fn main() {
    let string1 = String::from("abcd");

    // this code generates error
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }

    // println!("The longest sring is {}",result)

    //Lifetime ellistion

    //1. For a function each parameter that is used with reference gets its own lifetime parameter
    //2. If there is a single input lifetime parameter, that lifetime parameter is assigned to all the output lifetime parmeters
    //3. (only methods)If there are multiple input lifetime parameters but one of them is &self or &mut self the lifetime of that input is assigned to all the output parameters

    struct ImportantExcerpt<'a> {
        part:&'a str,
    }

    impl<'a> ImportantExcerpt<'a>{
        fn return_part(&self,announcement:&str)->&str{
            println!("announcement: {}",announcement);
            self.part
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // this functions has problems bcoz x and y may have different lifetime
    // since this functions takes in a borrowed reference and returns the same
    // its output value directly depends on the lifetime of its input
    // in the main function the lifetime of string2 ends before the result is printed
    // if string2 is the longest the return value in result will be erased from memory before reaching the print statement
    // hence the compiler gets confused with what the lifetime of result could be
    // we use lifetime trait to set the lifetime of op to be the least of input
    // LIFETIME OF RETURN IS ALWAYS TIED TO ONE OF THE INPUT PARAM TO THE FUNCTION
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
