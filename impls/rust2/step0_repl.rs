use std::io::{stdin, stdout, Write};


fn main() {

    loop {
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(bytes) => {
                if bytes <= 1 {
                    break println!("EOF Reached, Goodbye :)");
                } else {
                    println!("Your input: {}", my_rep(&mut buffer));
                    print!("user>");
                }
            }
            Err(error) => {
                println!("error: {}", error);
            }
        }
            stdout().flush().unwrap();
    }
    
}

pub fn my_read( input: &mut String) -> &mut String {
    input
}

pub fn my_eval(input: &mut String) -> &mut String {
    input
}

pub fn my_print(input: &mut String) -> &mut String {
    input
}

pub fn my_rep(input: &mut String) -> &mut String {
    my_read(input);
    my_eval(input);
    my_print(input);
    input
}