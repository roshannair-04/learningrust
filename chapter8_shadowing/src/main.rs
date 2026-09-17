// Shadowing in Rust allows you to declare a new variable with the exact same name as a previous variable using the let keyword again
// Shadowing is not the same as marking a variable mut or making a variable mutable


fn main() {
    println!("Hello, world!");
    let x=5;
    println!("the val of x is: {}",x);
    let x=x+1;
    println!("the val of x is: {}",x);
    let x=x*1000;
    // x=10;
    println!("the val of x is: {}",x);
    {
        let x=x*2;
        println!("the val of x is: {}",x);
    }
    // let mut spaces="  ";
    // spaces=spaces.len();
}
