use std::fmt::format;

fn main(){
    //=======================Strings=======================
    // Rust uses utf-8, hence each character can be of different sizes 1-4 bytes
    // utf-8 is backwards compatible with ASCII

    let s1 = String::from("Hello World");
    let s2="wonderful";
    let s3=String::from(s2);
    let s4=String::new();

    let mut string = String::from("foo");
    string.push_str(" bar");// for pushing strings
    string.push('!');//for pushing characters

    let s5=s1 + &s2;//taking ownership of s1 not s2

    let s5= format!("{}{}",s2,s3);//although no '&' is used ownership still remains with orignal when using format!

}