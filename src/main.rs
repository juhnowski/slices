fn main() {
    let s = String::from("Hello, World!");

    let hello = &s[0..5];
    let world = &s[7..13];

    println!("{:?}", hello);
    println!("{:?}", world);
}
