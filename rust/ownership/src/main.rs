fn main() {
    let title = String::from("Rust Ownership");
    let copy = title.clone();

    println!("original: {}", title);
    println!("cloned: {}", copy);

    let length = len(&title);
    println!("length: {}", length);

    let message = describe(&title);
    println!("{}", message);
}

fn len(value: &String) -> usize {
    value.len()
}

fn describe(value: &str) -> String {
    format!("{} bytes", value.len())
}
