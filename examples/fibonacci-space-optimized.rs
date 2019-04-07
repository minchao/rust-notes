// Fibonacci (space optimized)

fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut c: u64;

    if n == 0 {
        return a;
    }

    let mut i = 2;
    while i <= n {
        c = a + b;
        a = b;
        b = c;
        i = i + 1;
    }

    b
}

fn main() {
    let n = 10;
    let mut i = 0;
    while i < n {
        print!(" {}", fibonacci(i));
        i = i + 1;
    }
    print!("\n");
}
