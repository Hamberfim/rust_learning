fn main() {
    println!("Hello, from main!");
    // another_func(5);

    print_labeled_measurements(3, 'h');
}

// fn another_func(x: i32) {
//     println!("Hello from 'Another function' who param is {x}.");
// }

fn print_labeled_measurements(num_value: i32, unit_label: char) {
    println!("The measurement is: {num_value}{unit_label}");
}
