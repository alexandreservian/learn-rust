fn main() {
    let x = false;
    let y: bool = true;
    if x {
        println!("X is true!");
    }
    if y {
        println!("Y is true!");
    }

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
}