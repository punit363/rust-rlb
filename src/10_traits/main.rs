use std::{fmt::Display, iter::Sum};

struct Newsletter{
    author:String,
    headline:String,
    content:String,
}

struct Tweet{
    username:String,
    content:String,
    reply:bool,
    retweet:bool
}

impl Summary for Newsletter{
    fn summarise(&self)->String {
        format!("{}, by {}", self.headline, self.author)
    }
}

impl Summary for Tweet{
    fn summarise(&self)->String {
        format!("{}, by {}", self.content, self.username)
    }
}
trait Summary{
    // traits define a collection of method signatures to represent shared, abstract behavior
    // hence we are not defining the body for this method just its characterstic that it returns a string
    // fn summarise(&self)->String;

    //This does not have a default implementation hence need to specify in both the Tweets and Newsletters
    fn summarise(&self)->String;

    //This is default implementation of trait if not specified
    //At present both Tweet and Newletter overwrites this method 
    fn summarise_authore(&self)->String{
        format!("Read More...")
    }
}

//using trait in function========================================
// fn notify(item: &impl Summary){
//     println!("Breaking News!: {}",item.summarise())
// }

//Another way to write the same shit
fn notify<T:Summary>(item: &T){
    println!("Breaking News!: {}",item.summarise())
}

fn notify2(item1: &impl Summary, item2: &(impl Display + Summary)){
// item2 can only be something which has both the traits
}

// fn notify<T:Summary+Display>(item1: &T, item2: &T){
// //
// }

 
fn main(){

    let newsletter = Newsletter{
        author:String::from("John Does"),
        headline:String::from("The sky is falling!"),
        content:String::from("The sky is not actually falling."),
    };

    let tweet = Tweet{
        username:String::from("John@Does"),
        content:String::from("The sky is falling!"),
        reply:false,
        retweet:false
    };

    println!("Tweet Summary: {}",tweet.summarise());
    println!("Newsletter Summary: {}",newsletter.summarise());

    // can only return something with a summary trait i.e. tweet or newsletter
    // fn return_summarizable() -> impl Summary {
    //     Tweet{
    //         username:String::from("John@Does"),
    //         content:String::from("The sky is falling!"),
    //         reply:false,
    //         retweet:false
    //     };
    // }

    notify(&tweet);
}