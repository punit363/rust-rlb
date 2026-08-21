use std::collections::HashMap;

fn main(){

    let team_blue = String::from("Blue");
    let team_red = String::from("Red");

    let mut scores:HashMap<String, i32>=HashMap::new();

    scores.insert(team_blue, 114); //ownership moved to hashmap
    scores.insert(team_red, 194);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(x)=>println!("Score of team {} is {}", team_name, x),
        _=>println!("")
    }

    //Optionally enter value if not exist
    scores.entry(String::from("Yellow")).or_insert(44);
    scores.entry(String::from("Yellow")).or_insert(424);
      
    for (key , value) in &scores{
        println!("{} -> {}",key,value);
    }

}