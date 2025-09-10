
fn countupto(x:i32){
    let mut counter = 0;
    for i in 1..x+1{
        counter += i;
    }

    println!("{}", counter);
}

fn main() {
//call the function
    countupto(10);
    
}
    