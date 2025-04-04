use std::io;

fn main() {
    loop {
        println!("Please select the conversion type:");
        println!("1. Celsius to Fahrenheit conversion");
        println!("2. Fahrenheit to Celsius conversion");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        match choice.trim() {
            "1" => {
                let celsius = get_temperature_input("Please enter the Celsius temperature: ");
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("{:.2} Celsius = {:.2} Fahrenheit", celsius, fahrenheit);
                break;
            }
            "2" => {
                let fahrenheit = get_temperature_input("Please enter the Fahrenheit temperature: ");
                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!("{:.2} Fahrenheit = {:.2} Celsius", fahrenheit, celsius);
                break;
            }
            "3" => {
                println!("Exit the program.");
                break;
            }
            _ => println!("Invalid selection, please try again."),
        }
    }
}

fn get_temperature_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input, please enter a number."),
        }
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}
