fn main() {
    println!("Hello, my test!");
    let s =    String::from("123");
    let b = s.as_bytes();

    for u in b {
        println!("{}",u)
    }
}
