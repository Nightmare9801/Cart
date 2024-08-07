use crate::{cart::Cart, case::Case};

/// The `run_tui` function in Rust implements a text-based user interface for interacting with a cart
/// data structure, providing commands to add, remove, get, print, and display help information.
pub fn run_tui() {
    println!("Enter HELP to check the commands !!");
    let mut cart: Cart = Cart::new();
    loop {
        let input = get_input();
        if input.contains("HELP") {
            println!("ADD <Key> <String>        -> Insert A Case To The Database");
            println!("REMOVE_NUM <Key(Number)>  -> Remove A Case From The Database With Primitive Key");
            println!("REMOVE <Key(String)>      -> Remove A Case From The Database With Key");
            println!("GET_NUM <Key(Number)>     -> Get A Case From The Database With Primitive Key");
            println!("GET <Key(String)>         -> Get A Case From The Database With Key");
            println!("PRINT                     -> Print The Cart Cases");
            println!("HELP                      -> Display The Help Menu");
            println!("EXIT                      -> EXIT");
        } else if input.contains("PRINT") {
            cart.print();
        } else if input.contains("ADD") {
            let breaker: Vec<&str> = input.split(" ").collect();
            let mut str: String = breaker[2].trim().to_owned();
            for i in 3..breaker.len() {
                str += breaker[i].trim();
            }
            cart.insert(str, breaker[1].to_owned());
        } else if input.contains("REMOVE") {
            let breaker = match breaking(&input, 2) {
                Some(value) => value,
                None => continue,
            };
            cart.remove(breaker[1].trim().to_owned());
        } else if input.contains("REMOVE_NUM") {
            let breaker = match breaking(&input, 2) {
                Some(value) => value,
                None => continue,
            };

            let key_to_remove = match breaker[1].parse::<u64>() {
                Ok(key) => key,
                Err(_) => {
                    eprintln!("Warning: '{}' is not a valid u64. Item removal failed.", breaker[1]);
                    continue;
                }
            };
        
            cart.remove_primitive(key_to_remove);
        } else if input.contains("GET") {
            let breaker: Vec<&str> = match breaking(&input, 2) {
                Some(value) => value,
                None => continue,
            };
            let getter: Option<Case> = cart.get(breaker[1].to_owned());
            match getter {
                Some(x) => println!("{}", x.get_packet()),
                None => {
                    println!("Key is not present in the Cart.");
                    continue;
                } 
            }
        } else if input.contains("GET_NUM") {
            let breaker = match breaking(&input, 2) {
                Some(value) => value,
                None => continue,
            };

            let key_to_remove = match breaker[1].parse::<u64>() {
                Ok(key) => key,
                Err(_) => {
                    eprintln!("Warning: '{}' is not a valid u64. Item removal failed.", breaker[1]);
                    continue;
                }
            };
        
            let getter: Option<Case> = cart.get_primitive(key_to_remove);
            match getter {
                Some(x) => println!("{}", x.get_packet()),
                None => {
                    println!("Key is not present in the Cart.");
                    continue;
                } 
            }

        } else if input.contains("EXIT"){
            break;
        } else {
            println!("Wrong Command");
        }
    }
}

/// The `breaking` function in Rust takes a string input and splits it by spaces, returning a vector of
/// substrings if the number of substrings matches the specified index, otherwise it returns `None`.
/// 
/// Arguments:
/// 
/// * `input`: A reference to a `String` containing the input text that needs to be broken down into
/// separate parts.
/// * `i`: The parameter `i` in the function `breaking` is of type `usize` and represents the expected
/// number of elements after splitting the input string by spaces.
/// 
/// Returns:
/// 
/// The function `breaking` returns an `Option` containing a `Vec` of string slices (`&str`). If the
/// number of elements in the input string after splitting by spaces is not equal to the specified index
/// `i`, it will return `None` with a message "Wrong Number Of Inputs". Otherwise, it will return
/// `Some(breaker)` where `breaker` is the vector of string
fn breaking(input: &String, i: usize) -> Option<Vec<&str>> {
    let breaker: Vec<&str> = input.split(" ").collect();
    if !(breaker.len() == i) {
        println!("Wrong Number Of Inputs.");
        return None;
    }
    Some(breaker)
}

/// The `get_input` function in Rust reads a line of input from the user and returns it as a String.
/// 
/// Returns:
/// 
/// The `get_input` function returns a `String` value, which is the user input read from the standard
/// input (stdin).
fn get_input() -> String {
    let mut line: String = String::new();
    print!("\r");
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    line
}