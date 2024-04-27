use std::io;

fn main() {
    println!("Welcome to the fahrenheit converter");

    loop{
        println!("Enter fahrenheit value");
        let mut fahrenheit: String= String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");
        let fahrenheit:f32 = fahrenheit.trim().parse().expect("Not a number?");
        let celcius = fahrenheit_to_celcius(fahrenheit);
        println!("{fahrenheit}F is {celcius}C")
    }
   
}
const FAHRENHEIT_CONST:f32 = 5.0 / 9.0;
fn fahrenheit_to_celcius(fahrenheit:f32) -> f32 {
    (fahrenheit - 32.0) * FAHRENHEIT_CONST
}
