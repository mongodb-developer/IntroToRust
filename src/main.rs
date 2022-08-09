fn main() {
    let mut whom: &str = "world";
    println!("Hello, {whom}!");
    whom = "planet";
    println!("Bye, {whom}!");
    let x = String::from("This is not a &str");
    use_string(&x);
    println!("x = {x}");

    let times = "5";
    let times: Option<i32> = times.parse().ok();
    println!("times = {times:?}");
    if let Some(v) = times {
        println!("times = {v}");
    }
}

fn use_string(s: &str) {
    println!("Inside of use_string: s = {s}");
}
