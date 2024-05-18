fn main() {
    println!("Hello, world!");

    // Data Type in rust

    // Scalar variable typess

    // 1. unsigned interger: these are always -ve in nature
    // it can store size in u8, u16, u32, u64, u128 bits
    // it starts with "u"

    let unsigned: u8 = 2;

    // 2. singned integer: these have sign either +ve or -ve
    // it can store size in i8, i16, i32, i64, i128 bits
    // it starts with "i"
    let signed: i8 = -10;

    // 3. float: these are  the decimal values
    // Rust’s floating-point types are f32 and f64,
    // which are 32 bits and 64 bits in size, respectively.
    // it starts with "f"
    let float: f32 = 1.2;

    // 4. char
    //Note: if we want to use single char then use single quote and if we want
    // use a string then use double quote

    // single quote char value
    let letter = 'c';

    println!("string data types in  Rust");

    // double quote char value
    let letter2 = "char";

    //  5. boolean type

    println!("boolean data types in  Rust");

    let is_true: bool = true;

    println!(
        "unsigned: {}, sign: {}, float: {}, letter: {}, letter2: {}",
        unsigned, signed, float, letter, letter2
    );

    println!("isTrue: {}", is_true);

    // Compound types

    println!("Arrays in  Rust");

    // Arrays in rust

    // right way to create array

    let arr: [u8; 3] = [1, 2, 3];
    // [u8; 3]: u8- data type, 3- length of an array.

    println!("{:?}", arr); // this is how you print array

    let other_arr: [u8; 5] = [100; 5];
    // here [100; 5] - this means - array cantains 5 value and value to be print is 100 for five times

    println!("{:?}", other_arr);

    // Tuples

    println!("Functions in  Rust");

    // Functions in Rust

    println!("{}", is_even(1));

    pub fn is_even(num: u8) -> bool {
        let digit: u8 = num % 2;
        digit == 0 // returns bool if codition satisfies
    }

    // Mutable in rust

    let mut num: u8 = 5;
    num = 3;
    println!("{}", num);
}
