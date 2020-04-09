fn main() {
    println!("i8 = {} a {}", i8::min_value(), i8::max_value());
    
    println!("i16= {} a {}", i16::min_value(), i16::max_value());
    
    println!("i32= {} a {}", i32::min_value(), i32::max_value());
    
    println!("i64= {} a {}", i64::min_value(), i64::max_value());
    
    println!("u8 = {} a {}", u8::min_value(), u8::max_value());
    
    println!("u16= {} a {}", u16::min_value(), u16::max_value());
    
    println!("u32= {} a {}", u32::min_value(), u32::max_value());
    
    println!("u64= {} a {}", u64::min_value(), u64::max_value());

    let b: i8 = 1;
    println!("Uns: {}", b.count_ones());
    println!("Zeros: {}", b.count_zeros());
    

    let a: f32 = 3.549236;
    println!("Floor: {}", a.floor());
    println!("Ceil: {}", a.ceil());
    println!("Round: {}", a.round());
    println!("Truncate: {}", a.trunc());
    println!("Fractional: {}", a.fract());
    println!("Is Finite?: {}", a.is_finite());
    println!("Is Infinite?: {}", a.is_infinite());
    println!("Is NaN?: {}", a.is_nan());
}
