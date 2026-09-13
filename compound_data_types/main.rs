//types: arrays, tuples,slices,strings and slice strings

//arrays
fn main(){
    // let num:[i32;5]=[1,2,3,4,5];
    // println!("the num array: {:?}",num);
    // let fruits:[&str;3]=["apple","banana","orange"];
    // println!("the fruit array: {:?}",fruits);

    //Tuple
    // let human:[i32,str,bool]=[23,"banana",true];
    // let human=(23,"banana",true);
    // let human:(i32,String,bool)=(23,"banana".to_string(),true);
    // println!("human tuple: {:?}",human);
    // let my_mixtuple=(12,[1,2,3],"kratos",true);
    // println!("mix tuple: {:?}",my_mixtuple);

    //SLICES: [12,3,4,5,6]
    // let numslice:&[i32]=&[1,2,3,4,5];
    // println!("slices : {:?}",numslice);
    //String slice
    let mut stonecold:String=String::from("Hell, ");
    stonecold.push_str("yeah!");
    println!("Stonecold says: {}",stonecold);






}