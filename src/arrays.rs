// Arrays - fixed list where elemtns are the same data types

pub fn run() {
    println!("Arrays.rs start");
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5]; // can make mutable with mut

    // re assigning
    numbers[2] = 20;
    println!("{:?}", numbers); // gets all values
    println!("Single value: {}", numbers[0]);

    // get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated 51:56
}