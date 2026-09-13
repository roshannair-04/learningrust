//Primitive Data types
//int,float,bool,char

//Integer: signed integers [i8,i16,i32,i64,i128 can hold both + and - integers], whereas unsigned integers[u8,u16,u32,u64,u128] can hold only + integers

fn main(){

    // let x:i32=-42;
    // let y:u64=32;
    // println!("signed integer: {}", x);
    // println!("signed integer: {}", y);
    //trying the sign changes
    // let x:i32=42;
    // let y:u64=-32;
    // println!("signed integer: {}", x);
    // println!("signed integer: {}", y);
    //[putting number larger than range will cause compilation error]
    //floats
    // let e:f64=3.14;
    // println!("val of pi: {}",e);
    //bool
    // let is_snowing:bool=true;
    // println!("is it snowing? : {}",is_snowing);
    //CHAR
    let letter:char='a';
    println!("the first letter of the alphabets is: {}",letter);
}