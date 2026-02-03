trait Describable {
    fn describe(&self) -> String;
}

struct Book {
    title: String,
}

struct Car {
    brand: String,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("Book: {}", self.title)
    }
}

impl Describable for Car {
    fn describe(&self) -> String {
        format!("Car: {}", self.brand)
    }
}

fn print_desc<T: Describable>(item: &T) {
    println!("{}", item.describe());
}

fn main() {
    let book = Book {
        title: "The Rust Programming Language".to_string(),
    };
    let car = Car {
        brand: "Toyota".to_string(),
    };

    print_desc(&book);
    print_desc(&car);
}
