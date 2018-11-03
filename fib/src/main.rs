fn main() {
    println!("Generate the nth sequence of the fibonacci sequence for: ");

    let mut n = String::new();

    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read n from user input");

    let n: u32 = n.trim().parse().expect("Failed to parse n from user input");

    let fib = fib(n);
    let fmt = fib.to_string();
    println!("{}", fmt);
}

fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
