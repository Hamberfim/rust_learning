// control flow

fn main() {
    let number: i32 = 7;
    if number < 5 {
        println!("Condition was true.");
    } else {
        println!("Condition was false.");
    }

    if number != 0 {
        println!("Number is something other than zero.");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if in a let statement
    let condition: bool = false;
    let new_number: i32 = if condition { 4 } else { number };
    println!("The value of the number is {new_number}");
}
