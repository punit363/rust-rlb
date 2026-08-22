fn main(){

    //Generics in Functions============================
    let number_list = vec![34,40,52,76,21,88];

    let largest_number = get_largest(number_list);

    println!("The largest number is {}",largest_number);

    let char_list = vec!['a','c','k','p','t'];


    let largest_char = get_largest(char_list);

    println!("The largest char is {}",largest_char);

    //Generics in Structs==============================
    // struct Point{
    //     x:i32,
    //     y:i32
    // }
    // let p = Point{x:33,y:22};
    
    // struct Point<T>{
    //     x:T,
    //     y:T
    // }
    // let p = Point{x:33.5,y:22.1};

    struct Point<T,U>{
        x:T,
        y:U
    }
    let p1 = Point{x:33,y:22};
    let p2 = Point{x:33.5,y:22.1};
    let p3 = Point{x:33,y:22.1};
    let p3 = Point{x:'a',y:'b'};

    let p4 = p3.mix(p2);

    println!("X : {} Y : {}",p4.x,p4.y);

    //Generics in Methods
    // impl<U> Point<U>{
    //     //available to points where x and y are of same type
    //     fn x(&self)->&U{
    //         &self.x
    //     }

    // impl<f32> Point<f32>{
    //     //available to points of f32 type
    //     fn y(&self)->f32{
    //         self.y
    //     }

    // }

    //more complext
    impl<T,U> Point<T,U>{

        fn mix<V,W>(self, other:Point<V,W>)->Point<T,W>{
            Point{
                x:self.x,
                y:other.y
            }
        }
    }

    //Generics in Enums==============================
    enum Result <T,E>{
        Ok(T),
        Err(E)
    }


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