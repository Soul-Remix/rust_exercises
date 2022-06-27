use std::io;

fn main() {
    println!("Enter a positive number:");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("failed to read line");

    let number: u32 = number.trim().parse().expect("You should enter a number");

    let fib_number: u32 = fib(number);

    println!("Fibonacci number is: {}", fib_number)
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    return fib(n - 2) + fib(n - 1);
}
