fn main() {
    //loop keyword
//     loop{
//     println!("Hello, world!");
// }WILL print an infinite loop

// let mut counter=0;
// let result=loop{
//     counter+=1;
//     if counter==10{
//     break counter*2;
//     }
// };
// println!("The result is {}!",result);

// Loop labels for nested/multiple loops
//  let mut count = 0;
//     'counting_up:loop{
//         println!("count:{count}");
//         let mut remaining=10;
//         loop{
//             println!("remaining:{remaining}");
//             if remaining==9{
//                 break;
//             }
//             if count==2{
//                 break 'counting_up;
//             }
//             remaining-=1;
//         }
//         count+=1;
//     }
   
// WHILE LOOP
// let mut number=3;
// while number !=0{
//     println!("{number}");
//     number-=1;
//     // break;
// }
// println!("HEY");

//looping through an array/collection
let a=[1,2,3,4,5,6];
for fruit in a{
    println!("{fruit}");
}
}
