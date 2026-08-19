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

    //===============================================================================================
    //Ownership for complex data types moves with functions taking them in parameters
    let string1 = String::from("S T R I N G");

    let string2 =take_and_give_ownership(string1);
    
    take_ownership(string2);

    // println!("{}",string2)

    //No Ownership problem as simple datatype have their copies passed on in a function

    let p = 32;

    print_num(p);

    println!("{}",p);
    
}

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