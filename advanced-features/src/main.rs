pub trait HelloMacro {
    fn hello_macro();
}

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello Marco! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}