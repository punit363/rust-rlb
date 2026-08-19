//Structs are a way to define structure of data in Rust
struct User {
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool
}

fn main(){
    // Struct can be directly be invoked
    let mut user = User {
        email : String::from("znag123@gmail.com"),
        username : String::from("znag123"),
        sign_in_count:4,
        active:true
    };

    user.sign_in_count =5;

    // Can also be invoked Using a contructor function
    let user2 = create_user(String::from("john"), String::from("doe@mail.com"));

    // Struct also allows reusing data from other instances
    let user3 = User{
        email : String::from("henry@gmail.com"),
        username : String::from("henry123"),
        ..user2
    };
}

fn create_user(username:String,email:String)->User{
    User{
        username,
        email,
        sign_in_count:0,
        active:true
    }
}