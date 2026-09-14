//Functions
//entry point
// fn main(){


//     println!("hello boss!");
// }

//any function/variables should be written in 
// snake case eg hello_world[recommended]
//kebab case: hello-world
fn main(){


    hello_world();
    tell_height(182);
    human_id("roshan",22,182.0);
    let _x:i32={
        let price:i32=5;
        let qty:i32=10;
        price*qty
    };
    println!("the result is: {}",_x);
    // add(a:4,b:6);
    let y:i32=add(4,6);
    println!("val of y is: {}",y);
    //bmi
    let weight=115.0;
    let height=1.82;
    let bmi=calc_bmi(weight,height);
    println!("the bmi is: {}",bmi);

}

// const _i={
// //code
// };

fn hello_world(){
    println!("hello rust!");


}
//input vals func
fn tell_height(height:u32){
    println!("My height is: {}cm.",height);


}
//more than one value
fn human_id(name:&str,age:u32,height:f32){
    println!("My name is {} and my age is {} and i am {}cm tall.",name,age,height);
}

//Expression: anuything that returns a value
//Statement:anything that doesn't return a value
//Expression:5,true,add(3,4), if conditions
//

fn add(a:i32,b:i32)->i32{
    a+b
}

//control flow statements if and while statements
// BMI=weight(kg)/h(m)2

fn calc_bmi(weight_kg:f64,height_m:f64)->f64{
        weight_kg/(height_m*height_m)
}
