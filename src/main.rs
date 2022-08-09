fn main() {
    let mut whom: &str = "world";
    println!("Hello, {whom}!");
    whom = "planet";
    println!("Bye, {whom}!");
    let y: &str;
    let x = String::from("This is not a &str");
    y = &x;
    use_string(&x);
    println!("x = {x}");
    println!("y = {y}");
}

fn use_string(s: &str) {
    println!("Inside of use_string: s = {s}");
}
