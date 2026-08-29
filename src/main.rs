fn main() {
    let mut v1 = vec![1, 2, 4];

    // 1. Mutate each element in-place (+1)
    for value in v1.iter_mut() {
        *value += 1; // Dereference to mutate the value in-place
    }

    // 2. Print elements via shared references (&i32)
    for value in v1.iter() {
        print!("{} ", value);
    }
    println!();

    // 3. Consume the vector by value (into_iter)
    for mut value in v1.into_iter() {
        value = value + value;
        print!("{} ", value);
    }
    println!();
    
    // Note: `v1` cannot be used here anymore because `into_iter()` consumed it.
}