fn main() {
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
    spaces = spaces.len().to_string(); //spaces is now a string
    println!("The value of spaces is: {}", spaces);
}