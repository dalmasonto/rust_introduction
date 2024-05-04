fn main() {
    println!("Hello, world!");
    // let ione: i8 = 1;
    // let itwo: i16 = 1;
    // let ithree: i32 = 1;
    // let ifour: i64 = 1;
    // let ifive: i128 = 1;

    // let usix: u8 = 1;
    // let useven: u16 = 1;
    // let ueight: u32 = 1;
    // let unine: u64 = 1;
    // let uten: u128 = 1;

    // let u11: usize = 11;
    // let i12: isize = 12;

    // println!("Numbers: {}, {}, {}", ione, itwo, ueight);

    // let text_2 = String::from("Hello");

    add_numbers(127, 127);
}

fn add_numbers(num_1: i8, num_2: i8) -> i8 {
    let sum: i8 = num_2 + num_1;
    println!("Sum of 2 numbers:- {} + {} is: {}", num_1, num_2, sum);
    sum
}
