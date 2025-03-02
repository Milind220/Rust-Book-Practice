use std::io;

fn read_input() -> String{
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn f_to_c(temp: f64) -> f64{
    (5.0 / 9.0) * (temp - 32.0)
}

fn c_to_f(temp: f64) -> f64{
    ((9.0 / 5.0) * temp) + 32.0
}

fn main() {
    println!("This is a temperature converter program!");
    println!("Enter your temperature with the unit after, and this will return the converted value");
    println!("For Example: 42C or 69F");

    let raw_input = read_input();
    let input: &str = raw_input.trim(); 

    if input.is_empty() {
        println!("Please enter a value!");
        return;
    } 

    if let Some(last_char) = input.chars().last() {
        if last_char == 'C' || last_char == 'F' {
            let temperature_str = &input[..input.len() - 1];
            match temperature_str.parse::<f64>() {
                Ok(number) => {
                    if last_char == 'C' {
                        let result = c_to_f(number);
                        println!("{number}°C is {result}°F");
                    } else {
                        let result = f_to_c(number); 
                        println!("{number}°F is {result}°C");
                    }
                }
                Err(_) => {
                    println!("Could not read temperature value! Are you sure you entered as per the format?");
                }
            }
        } else {
            println!("Invalid unit! Please enter either C or F");
        }
    } 
}
