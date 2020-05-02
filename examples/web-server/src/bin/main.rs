// see https://doc.rust-lang.org/stable/book/ch20-00-final-project-a-web-server.html
use hello::ThreadPool;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    // 在 `127.0.0.1:7878` 上監聽傳入的 TCP stream
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 建立一個可設定 thread 數量的 pool
    let pool = ThreadPool::new(4);

    // 遍歷 connection attempts
    // 注意，這個範例只會兩個請求，接著就會執行 Graceful shutdown and cleanup，參考 `ThreadPool.Drop` 實現
    for stream in listener.incoming().take(2) {
        // 從 `Result` 取出 `TcpStream`，如果 `Result` 是 `Error`，自動呼叫 `panic!` 終止程式
        let stream = stream.unwrap();

        println!("Connection established!");

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    // 執行中的 worker，會被 `ThreadPool.Drop` 中的 `thread.join()` blocking 直到工作執行完成。

    println!("Shutting down.");
}

// 處理請求，`TcpStream` 在這裡宣告為可變的，是因為它會繼續在內部追蹤其返回了什麼資料
fn handle_connection(mut stream: TcpStream) {
    // 建立用來儲存讀取資料的緩衝區，這是一個簡化的實現，未考慮請求超過緩衝區大小的情況
    let mut buffer = [0; 512];
    // 從 stream 讀取 bytes，並放入緩衝區
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "assets/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(20));
        ("HTTP/1.1 200 OK\r\n\r\n", "assets/hello.html")
    } else {
        ("HTTP/1.1 404 Not Found\r\n\r\n", "assets/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    // 等待並阻塞程式繼續執行，直到所有的 bytes 都被寫入連線
    stream.flush().unwrap();
}
