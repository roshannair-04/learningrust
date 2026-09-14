// ownership, borrowing and reference

// Ownership
// every value has a single owner,it can't be owned by two people, when the owner is out of scope it will be deleted

fn main(){
    let s1=String::from("RUST");
    // here s1 is the owner of "RUST"
    // let len=calculate_length(&s1);
    // println!("the length is s1 is: {}",len);
    let s2=s1;//transferred TO s2
    //println!("{}",s1);//will return error as the ownership is transferred
    println!("{}",s2);

}   
// fn calculate_length(s:&String)->usize{
//     s.len()

// }

// Borrowing
// 