fn print_len(s: &String) {
    println!("Length: {}", s.len());
}

fn append_world(s: &mut String) {
    s.push_str(" World");
}

fn main() {
    let mut name = String::from("Hello");

    print_len(&name);

    append_world(&mut name);
    println!("{}", name);
}
