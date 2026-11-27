fn main() {
    //static variable: lives for the entire duration of the program and has a fixed memory address
    static APP_NAME: &str = "Learning Variables Declaration Rust";

    // 1. Static variable usage
    println!("Running: {}", APP_NAME);

    let mut x = 5; //can be changed because it is mutable
    let y = 10; //cannot be changed because it is immutable
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    
    x = 6; //change the value of x to 6
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    //unlike let, const variables are always immutable and must be annotated with a type
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //constant variable, cannot be changed
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    let x = 5; //shadowing, x is immutable again
    let x = x + 1; //shadowing, x is immutable again
    {
        let x = x * 2; //shadowing, x is immutable again
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);

    let spaces = "   "; //spaces is a string literal
    let spaces = spaces.len(); //shadowing, spaces is now a number
    println!("The value of spaces is: {}", spaces);

    let mut spaces = "   "; //spaces is a string literal and mutable
    let binding = spaces.len().to_string(); //using let binding to create a longer lived value, binding is now a string
    spaces = &binding; //spaces is now a string
    println!("The value of spaces is: {}", spaces);

    // --- ADDITIONAL VARIABLE DECLARATION CONCEPTS ---
    // 2. Explicit type annotation
    let explicit_number: i64 = 100; //explicitly specifying the data type
    println!("Explicit type value: {}", explicit_number);

    // 3. Late initialization (declare first, assign once later without 'mut')
    let exam_score: u8; //declared without an initial value
    exam_score = 95; //assigned once before being used, still immutable
    println!("Exam score: {}", exam_score);

    // 4. Destructuring (declaring multiple variables at once from a tuple)
    let (user_name, user_age) = ("Budi", 25); //unpacks values into separate variables
    println!("User: {}, Age: {}", user_name, user_age);

    // 5. Unused variable prefix (_) to prevent compiler warnings
    let _unused_variable = 42; //prefixed with underscore so the compiler ignores it
}