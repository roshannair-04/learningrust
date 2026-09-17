// Structs
// Structs are used to name and package related values similar to tuples.

fn main() {
    let rect:(i32,i32)=(200,500);
    // Struct
    struct Book{
        title:String,
        author:String,
        pages:u32,
        available:bool,
    }
    // #[derive(Debug)]
    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let mut user1=User{
        active:true,
        username:String::from("rosh1"),
        email:String::from("rosh@gmail.com"),
        sign_in_count:1,
    };
    user1.email=String::from("rosh2@gmail.com");
    println!("the user email is {}",user1.email);
    
    // println!("{user1:?}");


    // Returning a struct using function
    fn build user(email:String,username:String)->User{
        User{
            active:true,
            email,
            username,
            sign_in_count:1,
        }
    }
    //create instances from other instance
    let user2:User=User{
        email:String::from("another@gmail.com"),
        ..user1

    }
    
    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let white = Color(255,255,255);

    // Unit-Like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
