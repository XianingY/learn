fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10, 2).unwrap_or(0);
    println!("10 / 2 = {}", result);

    let err = divide(10, 0);
    match err {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
