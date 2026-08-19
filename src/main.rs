enum IpAddType {
    v4(u8,u8,u8,u8),
    v6(String)
}

enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangColor(i32,i32,i32)
}

//enums also have methods and associated functions
impl Message{
    fn some_function(){
        println!("This is an associated function for an enum")
    }
}
fn main(){
    let localhost = IpAddType::v4(127,0,0,1);
    Message::some_function();

    //========================== Option Enum =========================
    // Rust does not have the concept for null values
    // If there is any encounter with a null value
    // use Option Enum

    enum Option<T>{
        Some(T),
        None
    }

    // Option enum is set in rust by default no need to create one like above

    let x = 32;
    let y = Some(48);

    // let summation = x+y;
    // above will throw error because y can be an int or cannot be

    let sum = x + y.unwrap_or(0);

    println!("sum: {}",sum);

}