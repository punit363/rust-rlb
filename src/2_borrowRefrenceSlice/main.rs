fn main(){
    //---------Memory---------------
    // 1. Rust uses Stack and Heap
    // 2. Stack stores all the immutable data and refrence pointers for Mutable data in Heap
    // 3. Heap stores all the mutable data
    // 4. Reading and Writing to Stack is faster than Heap

    //-------Ownership Rules-----------
    // 1. Each value in Rust has a variable that is called its owner.
    // 2. There can be only one owner at a time.
    // 3. When the owner goes out of scope, the value is dropped. scope is defined by curly brackets

    //===================== Ownership for Simple and Complex data types ============================

    let mut x = 2;
    let y = x;

    println!("{}",x);
    x=4;
    println!("{}",x);
    println!("{}",y);
    //NOTE: for simple data types RUST copys the data not moves
    // This does not throw error because the value with x still remains with x
    // y recieves a copy of x not a refrence of x
    // updating x later does not affect y

    let s1 = String::from("Hello");
    let s2 = s1;

    // println!("{}",s1);
    //NOTE: for complex data types RUST moves the data not copys or shares reference
    // This throws error because s1 has lost its refence to s2
    // s2 did not recieve a copy as it would waste memory
    // s2 did not share a refrence with s1 bcoz it would violate single ownership rule

    let s3 = s2.clone();
    println!("{}",s2);
    // use clone to create a second copy mannually for s3 letting s2 retain the orignal

    //================================ Ownership In function Calls ===============================

    // Ownership for complex data types moves with functions taking them in parameters
    let string1 = String::from("S T R I N G");

    let string2 =take_and_give_ownership(string1);
    
    take_ownership(string2);

    // println!("{}",string2)

    //No Ownership problem as simple datatype have their copies passed on in a function

    let p = 32;

    print_num(p);

    println!("{}",p);

    //====================================== Refrences ======================================

    // What if we don't want to share ownership?
    // Share a refrence then
    // Ownership remains but references are immutable hence readonly data

    let greetings1: String = String::from("Konichiwa");

    print_greeting(&greetings1);
    println!("{}",greetings1);

    //================================== Mutable Refrences ======================================

    // What if we don't want to share ownership and also mutate data?
    // Share a mutable refrence then
    // Ownership remains but only 1 mutable reference can be created

    let mut greetings2: String = String::from("Konichiwa");

    print_mutated_greeting(&mut greetings2);
    println!("{}",greetings2);

    // NOTE: MUTABLE REFRENCES CANNOT BE CREATED IF A IMMUTABLE REFRENCE ALREADY EXISTS IN THAT SCOPE
    // BCOZ IMMUTABLE REFERENCES DO NOT EXPECT THE DATA TO CHANGE
    // BUT IF THE SCOPE OF IMMUTABLE REFERENCE ENDS THEN WE CAN CREATE A MUTABLE REFRENCE

    let mut s = String::from("mutable string");

    let s1 = &s;
    let s2 = &s;

    println!("Scope for immutable reference ends here: s1-{}, s2-{}",s1,s2);

    // SCOPE OF A VAR STARTS WHEN IT IS INITIALISED AND ENDS THE LAST TIME IT IS USED
    // HENCE S1 AND S2 DONOT THROW ERR AND A MUTABLE REFERENCE CAN BE CREATED
    let s3 = &mut s;
    s3.push_str(" mutated");
    println!("{}",s3);

    //================================== Dangling Refrences ======================================
    
    // fn dangling_refrence()->&String{
    //     let s = String::from("ello");
    //     return &s;
    // }

    // Above function is invalid as it tried to pass a refrence of 's' but as the function call completes Rust deleted s 
    // which creates a Dangling Refrence which Ofcourse Rust doesn't let happen

    // let str = dangling_refrence();

    //======================================== Slices ============================================

    // What is you only want to reference a part of string not the whole

    let mut s = String::from("Hello World");

    let hello = &s[..5]; // &str is datatype -> string slice
    let world = &s[6..];

    let word="word";// string literals are actually string slices // stored directly in the binary

    // slice can also be used with arrays

    let marks = [100,94,85,78];

    let top_two = &marks[0..2];
}


//============ Ownership ===========
fn print_num(p:i8){
    println!("{}",p)
}

fn take_and_give_ownership(string:String)->String{
    println!("take_and_give_ownership => {}",string);
    return string;
}

fn take_ownership(string:String){
    println!("take_ownership => {}",string);
}

//============ Refrences ===========
fn print_greeting(greet:&String){
    println!("print_greeting => {}",greet);
    // greet.push_str("minnasan"); throws an error as references are immutable
}

//============ Mutable Refrences ===========
fn print_mutated_greeting(greet:&mut String){
    greet.push_str(" minnasan"); 
    println!("print_mutated_greeting => {}",greet);
}

