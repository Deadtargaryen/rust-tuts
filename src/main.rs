use std::io;

fn main() {
    
    let mut n = String::new();

    println!("Enter a number");

    io::stdin().read_line(&mut n).expect("expected user input");

    println!("you have entered: {}", n.trim().parse::<i32>().unwrap());

}
    