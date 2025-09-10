
fn countupto(x:i32) -> i32{
    let mut counter = 0;
    for i in 1..x+1{
        counter += i;
    }

    // println!("{}", counter);
    return counter;
}

fn main() {
//call the function
   let res =  countupto(10);
    println!("{}", res);
}
    