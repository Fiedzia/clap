// Start with bringing the trait into scope.
use std::str::FromStr;

// Add clap like normal
use clap::{arg, App};

// Define your enum
enum Vals {
    Foo,
    Bar,
    Baz,
    Qux,
}

// Implement the trait
impl FromStr for Vals {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Foo" => Ok(Vals::Foo),
            "Bar" => Ok(Vals::Bar),
            "Baz" => Ok(Vals::Baz),
            "Qux" => Ok(Vals::Qux),
            _ => Err("no match"),
        }
    }
}

fn main() {
    // Create the application like normal
    let m = App::new("myapp")
        // Use a single positional argument that is required
        .arg(
            arg!(<type> "The type to use")
                // Define the list of possible values
                .possible_values(["Foo", "Bar", "Baz", "Qux"]),
        )
        .get_matches();

    // Note that you don't have to specify the type since rustc can infer it for you
    let t = m
        .value_of_t("type")
        .expect("'type' is required and parsing will fail if its missing");

    // Now we can use our enum like normal.
    match t {
        Vals::Foo => println!("Found a Foo"),
        Vals::Bar => println!("Found a Bar"),
        Vals::Baz => println!("Found a Baz"),
        Vals::Qux => println!("Found a Qux"),
    }
}
