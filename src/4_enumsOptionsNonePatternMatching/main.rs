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

    //========================== Pattern Matching =========================
    enum Coin{
        Penny,
        Nickle,
        Dime,
        Quarter
    }

    fn value_in_cents(coin:Coin)->u8{
        match coin {
            Coin::Penny =>{
                println!("Its a penny");
                1
            },
            Coin::Nickle =>{
                println!("Its a Nickle");
                1
            },
            Coin::Dime =>{
                println!("Its a Dime");
                1
            },
            Coin::Quarter =>{
                println!("Its a Quarter");
                1
            },
        }
    }

    value_in_cents(Coin::Penny);

    // Pattern Matching with Option Enum
    let some_num = Some(4);
    match some_num {
        Some(3)=>println!("The number is {}",3),
        _ =>println!("The number is not {}",3) //in any case other than 3
    }
}