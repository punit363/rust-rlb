fn main(){

  let a = [1,4,7];

  //================================= VECTORS =================================

  //Initialising
  let mut v1:Vec<i32> = Vec::new();
  v1.push(23);
  v1.push(-3);
  v1.push(-12);

  let mut v2 = vec![12,-4,7,9,11];

  // unsafe data accessing
  // thows error at runtime if invalid index is used

  let third = &v2[2]; // refrencing data in vector
  // v2.push(8); // this is a problem as the immutable reference above "third" does not has its scope ended
  println!("The third element of vector v2 is {}",third);

  // safe data accessing

  match v2.get(5){
    Some(el)=>  println!("The fifth element of vector v2 is {}",el),
    None=>  println!("The fifth element of vector v2 does not exist ")
  }

  //iterating a vector
  for i in &mut v2{
    *i +=10;
  }

  for i in &v2{
    println!("{}",i)
  }

  // What if we can to store different types of element in vector
  // we can create an enum with different types

  enum SpreadSheetCell{
    Int(i32),
    Float(f64),
    Text(String)
  }

  let row = vec![
    SpreadSheetCell::Int(34),
    SpreadSheetCell::Text(String::from("blue")),
    SpreadSheetCell::Float(3.21),
    SpreadSheetCell::Int(31),
  ];

  match &row[3]{
    SpreadSheetCell::Int(i)=>println!("This one is an Integer : {}",i),
    _=>println!("This one is not an Integer")
  }
}