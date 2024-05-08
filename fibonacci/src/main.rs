use std::io;
use std::collections;


fn main() {

    println!("Welcome to Fibonacci calculator");
    loop{
        println!("Enter your input");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input).expect("Failed to read input");
        let input = input.trim().parse().expect("Not a number?");
        let mut fib_store:collections::HashMap<i128,i128> = collections::HashMap::new();
        let result = calculate_fibonacci(input,&mut fib_store);
        println!("The fibonacci of {input} is {result}!");
    }
}

fn calculate_fibonacci(fib: i128, store:&mut collections::HashMap<i128,i128>) -> i128{
    if store.contains_key(&fib){
        return *store.get(&fib).unwrap();
    }

    if fib <= 0 {
        return 0
    } 
    if fib == 1{
        return 1
    }
    let result = calculate_fibonacci(fib - 1,store) + calculate_fibonacci(fib - 2,store);
    store.insert(fib, result);
    result
}