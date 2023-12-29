fn main() {
    // mutable and immutable
    let mut x = 5; // the data type of x can not change, only its value
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // shadowing
    {
        // scope block
        let y = 10;
        println!("Outer Scope: The value of y is {y}");
        let y = y + 5; // shadow
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
}
