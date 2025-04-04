use std::io;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_iterative(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn main() {
    let mut input = String::new();
    println!("Please enter a number to calculate its Fibonacci value:");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let n: u32 = input.trim().parse().expect("Please enter a valid number");
    println!("Fibonacci of {} is {}", n, fibonacci(n));
    println!("Fibonacci of {} (iterative) is {}", n, fibonacci_iterative(n));
}
