use std::thread;
use std::sync::{mpsc, Arc, Mutex};

// Channel 傳輸的事件，`NewJob` 表示有新的工作，`Terminate` 表示 thread 應該退出循環終止執行
enum Message {
    NewJob(Job),
    Terminate,
}

// 類型別名，參考 19.3
// `Box<T>` 智能指針，允許將值放在 heap 上而不是 stack，參考 15.1
type Job = Box<dyn FnOnce() + Send + 'static>;

// Thread pool 管理一組預先分配並等待處理工作的 threads，它允許我們能夠平行處理請求的連線，來提高伺服器的效能。
// 當程式收到一個新工作時，thread pool 中的一個 thread 會被分配並執行工作，
// 而工作完成後，thread 便返回 thread pool 內繼續等待新的工作。
//
// 在 pool 中的 threads 會被限制在一個較少的數量，以防止 Denial of Service (DoS) 攻擊；
// 如果程式為每一個請求都建立新的 thread，那麼遭遇到大量請求時就會耗盡伺服器資源而導致程式終止執行。
pub struct ThreadPool {
    // Pool，使用 Vector 類型來儲存 workers，參考 8.1
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // 確保輸入值在允許的範圍，threads 不會是負數
        assert!(size > 0);

        // 建立跨 thread 間分享訊息用的 channel，`mpsc` 是 multiple producer, single consumer 的縮寫，
        // `ThreadPool` 是發送端，而 `Worker` 是接收端
        // 參考 16.2
        let (sender, receiver) = mpsc::channel();
        // Channel 的實現是 `mpsc`，需要使用 `Arc` 及 `Mutex`，讓 `receiver` 可以在多個 worker 間共享，
        // 參考 16.3
        let receiver = Arc::new(Mutex::new(receiver));

        // 使用 `with_capacity` 來建立 vector，由於 `size` 已知，所以我們可以預先分配空間
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // 建立 worker，並將其放到 pool
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    // 執行以 Closure 包裝的工作。
    // 參考 [13.1 Storing Closures Using Generic Parameters and the Fn Traits](https://doc.rust-lang.org/stable/book/ch13-01-closures.html#storing-closures-using-generic-parameters-and-the-fn-traits)
    //   - `FnOnce()`，只會執行一次，沒有參數也沒有返回值的 Closure
    //   - `Send`，將 Closure 從一個 thread 轉移到另一個 thread
    //   - `'static`，由於不曉得 thread 會執行多久，使用靜態生命週期，參考 10.3
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// 實現 `Drop`，當 `ThreadPool` 被丟棄時，應該 `join` 等待 thread 結束，以確保其完成當前的工作（參考範例 16-2）
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // `Option.take` 會取得 `Some` 而留下 `None`，`None` 表示 thread 已被被清理過
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// Worker 會建立一個不斷執行的 thread 來處理工作，並透過 channel 接收來自 pool 的工作通知
struct Worker {
    id: usize,
    // 使用 `Option`（參考 6.1）讓 `ThreadPool.Drop` 的 `join` 操作可以取得 `thread` 所有權，參考範例 17-15
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // 從 `receiver` 收到要執行的訊息
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
