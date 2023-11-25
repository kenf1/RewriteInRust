use std::io;

fn main(){
    let mut input_user = String::new();

    //username
    println!("Enter username:");
    io::stdin().read_line(&mut input_user).expect("Failed to read input.");

    //prompt pw if username matches
    match input_user.trim(){
        "user" => {
            println!("Enter password:");

            let mut input_pass = String::new();
            io::stdin().read_line(&mut input_pass).expect("Failed to read input.");
            
            match input_pass.trim(){
                "pass" => println!("Success!"),
                _ => println!("Incorrect password. Please try again.")
            }
        },
        _ => println!("Entered username not found")
    }
}