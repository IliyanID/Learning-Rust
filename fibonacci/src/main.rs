use std::io;

fn main() {
    println!("Welcome to Fibonacci calculator");
    loop{
        println!("Enter your input");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input).expect("Failed to read input");
        let input = input.trim().parse().expect("Not a number?");
        let result = calculate_fibonacci(input);
        println!("The fibonacci of {input} is {result}!");
    }
}

fn calculate_fibonacci(fib: i32) -> i32{
    if fib <= 0 {
        return 0
    } 
    if fib == 1{
        return 1
    }
    return calculate_fibonacci(fib - 1) + calculate_fibonacci(fib - 2)
}