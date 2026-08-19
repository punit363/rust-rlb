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
}