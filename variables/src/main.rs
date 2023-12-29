fn main() {
    // mutable and immutable
    let mut x = 5; // the data type of x can not change, only its value
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // shadowing
    {
        // scope block
        let y = 10; // integer
        println!("Outer Scope: The value of y is {y}");
        let y = y + 5; // shadow : integer
        println!("First Shadow: The value of y is {y}");
        {
            // scope block change
            let y = y / 2;
            println!("Second Shadow: Inner Scope: The value of y is {y}");
        }

        println!("Back to outer scope shadow: The value of y is {y}");
    }
    // shadowing also allows for data type changes
    let spaces = "     "; // five spaces- string type
    println!("{}", spaces); // blank five spaces
    let spaces = spaces.len(); // numeric type
    println!("{}", spaces); // the length

    // mathematical operators
    // addition
    let sum = 5 + 10;
    println!("{}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{}", difference);

    // multiplication
    let product = 4 * 30;
    println!("{}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);
    let truncated = -5 / 3; // Results in -1
    println!("{}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("{}", remainder);

    let t: bool = true;
    println!("{}", t);

    // tuple
    let tup1: (char, i32, f32, i64) = ('a', 16, 199.0987, 10000);
    println!("{:?}", tup1); // {:?} special format
    let tup2: (char, u8, u8, i128) = ('s', 255, b'A', 1_000_000_000);
    println!("{:?}", tup2); // {:?} special format

    // destructuring
    let tup3 = (500, 6.4, 1);
    let (x, y, z) = tup3;
    println!("The value of x, y and z is: {x}, {y} and {z}");
    // access tuples via index value
    let my_tuples: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = my_tuples.0;
    let six_point_four = my_tuples.1;
    println!("{five_hundred}, {six_point_four}");

    // arrays
    let my_arr1 = [17, 28, 3, 44];
    println!("Array: {:?}", my_arr1);
    println!("Array value at index 1: {:?}", my_arr1[1]);
    let my_rgb_color: [u8; 3] = [128, 3, 90]; // purple
    println!("RGB Value: {:?}", my_rgb_color);

    // tmp
    let message = "The temperature today is:";
    let x: (&str, u8) = (message, 100);
    println!("{} {}", x.0, x.1);
}
