// Fibonacci (vector)

fn main() {
    let n = 10;
    let mut v = vec![0, 1];
    let mut i = 2;
    while i < n {
        let x = &v[i - 1] + &v[i - 2];
        v.push(x);
        i = i + 1;
    }
    println!("{:?}", v);
}
