use std::{fs::File, io::{self, Error, ErrorKind, Read}};

fn c(num:i32){
if num==32{
    panic!("The program panicked! Do not pass in 32.")
    //RUST_BACKTRACE=1 cargo run
}
}

fn b(){
    c(32);
}

fn a(){
    b();
}

fn  main(){
    // Panic and backtrace
    // a();

    // Result Enum
    enum Result<T, E>{
        Ok(T),
        Err(E)
    }

    let file = File::open("hello.txt");

    // let f = match file{
    //     Ok(f)=>f,
    //     Err(Error)=>panic!("Problem opening the file: {:?}",Error)
    // };


    // Nested error handling
    /*
    let file = match File::open("hello.txt"){
        Ok(f)=>f,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound=>match File::create("hello.txt"){
                Ok(fc)=>fc,
                Err(err)=>panic!("Problem creating new file!{:?}",err),
            },
            other_error=>panic!("Problem opening file {:?}",other_error),
        }
    };
    */

    //concise way to write above code
    /*
    let file = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind()==ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error|{
                panic!("Problem creating new file!{:?}",error)
            })
        }else{
            panic!("Problem Opening file {:?}",error)
        }
    });
    */


    // unwrap
    // feeds the value to var is OK else panics with the error
    // let file = File::open("helloworld.txt").unwrap();

    // expect
    // feeds the value to var is OK else panics with the error message of our choosing
    let file = File::open("helloworld.txt").expect("Failed to open file helloworld.txt");

}

//========================== Error Propogation ==========================
fn read_user_name_from_file()-> Result<String, io::Error>{
    /*
    let f = File::open("hello.txt");

    let mut f= match f{
        Ok(file)=>file,
        Err(error)=>return Err(error),
    };

    let mut name = String::new();

    match f.read_to_string(&mut name){
        Ok(_)=>Ok(name),
        Err(err)=>Err(err),
    }
    */

    //Consise "?" does the same thing as unwrap
    /*
    let mut f = File::open("hello.txt")?;
    let mut s =String::new();
    f.read_to_string( &mut s)?;
    Ok(s)
    */

    //more concise
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}