// Fibonacci (use recursion)

fn fibonacci(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    fibonacci(n - 2) + fibonacci(n - 1)
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
