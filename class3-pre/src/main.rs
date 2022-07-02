#[derive(Debug)]
struct Rectangle{
    width: u32,
    height:u32,
}
impl Rectangle{
    fn area(&self)->u32{
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size:u32)->Rectangle{
        Rectangle{
            width:size,
            height:size,
        }
    }
}
fn main() {
    let rect1=Rectangle{
        width:10,
        height:20,
    };
    let rect2=Rectangle{
        width:3,
        height:5,
    };
    let sq=Rectangle::square(12);
    println!("{:?}",rect1);
    println!("{:?}",sq);
    println!("{}",rect1.area());
    println!("{}",rect2.can_hold(&rect1));
    
}

// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }



// ___________________________________________________________________________________________________




// fn main() {
//     let mut shopping_list: Vec<&str> = Vec::new();
//     shopping_list.push("milk");
//     println!("{:?}",shopping_list);
// }



// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email:String, username: String)-> User{
//     User
//     {
//         email,
//         username,
//         active:true,
//         sign_in_count:1,
//     }
// }

// fn main(){
    
//     build_user(String::from("aha@duckmail.com"),String::from("Aha"));

// }