use std::{env, os::windows::process};

fn main() {

    //filename.exe 
    
    let mut args = env::args().collect::<Vec<String>>();

    if args.len()!=3{
        println!("[+] Usage: filename.exe arg1 arg2 arg3");
        std::process::exit(0);
    }

    println!("{:?}", args);
}
    