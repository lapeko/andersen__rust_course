use std::io;

fn main() {
    let mut user_input = String::new();

    println!("Write down something");

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            println!("You wrote \"{}\"", user_input.trim())
        },
        Err(err) => {
            println!("Something went wrong {}", err)
        }
    }

    // printing values
    println!("{0} has a {1} and {0} also has a {2}", "Vitali", "cat", "dog");
    println!("{name} {surname}", name="Jim", surname="Carrey");
    println!("Debug an array: {:?}", [1, 2, 3]);
    println!("Binary {:b}, Octal {:o}, Hex {:x}", 5, 5, 5);
}
