fn main() {
    println!("Hello, world!");
    // signed integer
    // i8, i16, i64, i128
    let signed: i8 = -100;

    // float is used for decimals
    let float: f32 = 1.0;

    let is_true: bool = true;
    
    println!( " sign: {}: float: {} ", signed, float );

    println!("is True: {}", is_true);

    let arr: [u8; 3] = [1, 2, 3 ];
    let other_arr: [u8;5] = [100; 5];

    println!("index: {}, length: {}", arr[0], other_arr.len());

    println!("{:?}", other_arr);

    let tuple: (u8, bool, f32) = (5, true, 2.1);

    let tuple2 = (3, 5);

    println!("{:?}",tuple2)
}
