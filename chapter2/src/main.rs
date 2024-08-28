use std::io;@

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
}
