fn main(){
    let number_list = vec![34,40,52,76,21,88];

    let largest_number = get_largest(number_list);

    println!("The largest number is {}",largest_number);

    let char_list = vec!['a','c','k','p','t'];


    let largest_char = get_largest(char_list);

    println!("The largest char is {}",largest_char);

}

// fn largest_number(number_list: Vec<i32>) -> i32 {
//     let mut largest_number = number_list[0];
    
//     for number in number_list{
//         if number>largest_number{
//             largest_number=number;
//         }
//     }
//     largest_number
// }


//modifying the above function to take in any generic input
// T is a generic just a representation, it could have been U, R, TYPE etc
// PartialOrd(T can be ordered) / Copy(T can be copied) are traits
fn get_largest<T:PartialOrd+Copy>(number_list: Vec<T>) -> T {
        let mut largest = number_list[0];
        
        for number in number_list{
            if number>largest{
                largest=number;
            }
        }
        largest
 }