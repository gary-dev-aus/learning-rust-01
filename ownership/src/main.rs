fn main() {
    let mut s = String::from("hello");

    let r1 = &s;

    println!("{r1}");
    println!("{r1}");

    let r2 = &mut s;
    println!("{r2}");

    println!("{s}");
}
