fn main() {
    println!("Hello, from main!");
    // another_func(5);

    print_labeled_measurements(3, 'h');

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

// fn another_func(x: i32) {
//     println!("Hello from 'Another function' who param is {x}.");
// }

fn print_labeled_measurements(num_value: i32, unit_label: char) {
    println!("The measurement is: {num_value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    // an expression, no semicolon!
    x + 1
}
