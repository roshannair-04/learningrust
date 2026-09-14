fn main(){

    let mut _x=5;
    let _r=&mut _x;
    *_r+=1;
    *_r-=3;
    println!("the value of x: {}",_x);
    println!("the value of r: {}",_r);
}