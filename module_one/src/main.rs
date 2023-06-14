struct Rust(String);

impl Rust {
    fn new() -> Self {
        Rust(String::from("Rust"))
    }

    fn print() {
        let rust = Rust::new();
        println!("{}", rust.0);
    }
}

fn main() {
    // let r = Rust::new();
    Rust::print();
    
}
