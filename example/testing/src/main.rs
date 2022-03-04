fn calling_fun(a: String, b: String) -> String {
    // a.push_str(&b);
    println!("{} {}", a, b);
    return a;
}

fn main() {
    let mut x = String::from("Hello cs");
    x = calling_fun(x, String::from("Rust"));
    println!("The value of x is: {}", x);
}
