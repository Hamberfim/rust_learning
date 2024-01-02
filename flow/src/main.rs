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

    // loops
    let mut counter: i32 = 0;
    let result = loop {
        counter += 1;

        println!("The counter: {counter}");

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // nested loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
