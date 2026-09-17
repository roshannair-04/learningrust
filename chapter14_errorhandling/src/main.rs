// //approach 1
// enum Option<T>{//define option type
//     Some(T),//reps a value
//     None,    //represents no value
// }  
// //aproach 2
// enum Result<T,E>{
    
//     Ok(T),
//     Err(E),
// }
fn divide(denominator:f32,numerator:f32)->Option<f32>{
    if denominator==0.0{
        None
    }else{
        Some(numerator/denominator)
    }
    
}
fn main() {
    let result=divide(0.0,10.0);
    match result{
        Some(x)=>println!("Result is {}",x),
        None=>println!("can't divive by zero!"),
    }
}
