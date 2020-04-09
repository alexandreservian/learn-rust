fn main() {
    let a:char = '\u{2764}';
    let b:char = '9';
    let c:char = '0';

    println!("{} is a digital? {}", a ,a.is_digit(10));
    println!("{} is a digital? {}", a ,a.is_digit(2));

    println!("{} is a digital? {}", b ,b.is_digit(10));
    println!("{} is a digital? {}", b ,b.is_digit(2));

    println!("{} is a digital? {}", c ,c.is_digit(10));
    println!("{} is a digital? {}", c ,c.is_digit(2));


    let d: char = '→';
    let repr: String = d.escape_unicode().collect();
    println!("{}", repr);

    println!("Uppercase -> {}", 'a'.is_uppercase());
    println!("Lowercase -> {}", 'a'.is_lowercase());
    println!("Whitespace -> {}", 'a'.is_whitespace());
    println!("Alphanumeric -> {}", 'a'.is_alphanumeric());
    println!("Numeric -> {}", 'a'.is_numeric());
}
