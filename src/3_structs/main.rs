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

    // Tuple structs
    // Structs can also be created without naming their properties

    struct Color (i16,i16,i16);
    struct Point (i16,i16,i16);

    struct Rectangle {
        height:u32,
        width:u32
    }

    // implementation blocks are created for structs to house their functions and methods
    impl Rectangle{
        // &self is the instance of struct this method is called on
        // we can also take mutable refrences &mut self
        // METHODS ALWAYS HAVE FIRST PARAMETER AS SELF
        fn area(&self)->u32{
            self.width*self.height
        }

        fn can_hold_other_rect_inside(&self,other_rect:&Rectangle)->bool{
            self.area() > other_rect.area()
        }
    }

    // we can create multiple implementation blocks for a single struct
    impl Rectangle{
        //ASSOCIATED FUNCTION - does not bind to a particular instance of a function
        fn sqaure(size:u32)->Rectangle{
            Rectangle { height: size, width: size }
        }
    }

    let rect1 = Rectangle{
        height:32,
        width:45
    };

    println!("The area of given rectangle1 is {} squate pixels",rect1.area());
    
    let rect2 = Rectangle{
        height:32,
        width:40
    };

    println!("The area of given rectangle2 is {} squate pixels",rect2.area());

    println!("Rect1 can hold Rect2 inside : {}",rect1.can_hold_other_rect_inside(&rect2));

    let rect3 = Rectangle::sqaure(12);

}

fn create_user(username:String,email:String)->User{
    User{
        username,
        email,
        sign_in_count:0,
        active:true
    }
}