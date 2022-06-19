fn main() {

    //uint and int types
    let int_32_sample1 : i32 = 32i32;
    let uint_32_sample1 : u32 = 32;
    let uint_32_sample2 = 32_000u32;

    println!("Int 32 sample 1 = {}", int_32_sample1);
    println!("UInt 32 sample 1 = {}", uint_32_sample1);
    println!("UInt 32 sample 2 = {}", uint_32_sample2);

    //Float types
    let float_32_sample : f32 = 32.6;
    let float_64_sample : f64 = 32.5;

    println!("Float 32 sample = {}", float_32_sample);
    println!("Float 64 sample = {}", float_64_sample);

    //Boolean type
    let bool_sample : bool = true;
    println!("Bool sample = {}", bool_sample);

    //Char type
    let char_sample1 : char = 'A';
    let char_sample2 : char = '😻';
    println!("Char sample 1 = {}", char_sample1);
    println!("Char sample 2 = {}", char_sample2);

    //Tuple type
    let tuple_sample : (i32, f64, char) = (10, 10.5, '1');
    let (x, y, z) = tuple_sample;

    println!("Tuple sample = ({}, {}, {})", x, y, z);
    println!("Tuple sample = ({}, {}, {})", tuple_sample.0, tuple_sample.1, tuple_sample.2);

    //Array type
    let array_sample = [0, 1, 2, 3, 4, 5];
    let array_sample2 : [i32; 5] = [1, 2, 3, 4, 5];
    let array_sample3 = [3; 5];
    println!("Array sample1, idx 0 = {}", array_sample[0]);
}
