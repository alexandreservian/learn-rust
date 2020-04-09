fn main() {
    let a = ["alexandre", "prado", "servian"];
    println!("nome do programador {} {} {}", a[0],a[1],a[2]);

    println!("tamanho do array {}", a.len());


    let b = ['a', 'b', 'c', 'd'];
    for x in b.iter() {
        println!("{}", x);
    }
}
