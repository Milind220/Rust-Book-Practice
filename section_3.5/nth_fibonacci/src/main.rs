use std::io;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Expected a number!");

    input
}

fn nth_fibonacci_number(index: i32) -> i32 {
    if index == 1 {
        return 0;
    } else if index == 2{
        return 1;
    } else {
        let mut prev_term = 0;
        let mut current_term = 1;
        for _num in 0..index-2 {
            let new_term = prev_term + current_term;
            prev_term = current_term;
            current_term = new_term;
        }
        current_term
    }
}

fn main() {
    println!("This program generates the nth Fibonacci number");
    println!("Please enter a number (Assume 1 indexing): ");

    let number: i32 = read_input().trim().parse::<i32>().expect("Expected a number!");
    let result = nth_fibonacci_number(number);
    println!("The {number} Fibonacci term is {result}");
}
