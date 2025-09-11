
pub fn countupto(x:i32) -> i32{
    let mut counter = 0;
    for i in 1..x+1{
        counter += i;
    }

    // println!("{}", counter);
    return counter;
}


pub fn adder(a:&Vec<i32>){
    println!("{:?}", a);
}


fn randongeneration() ->String {
    let name =  "tech69";
    return name.to_string();
}

fn main() {
//call the function
//    let res =  countupto(10);

// let arr = [5,6,7,8,9];
// adder(&arr);


    // let v1 = (0..50).collect::<Vec<i32>>();
    // adder(&v1);

    // println!("{:?}", v1);


    println!("{}", randongeneration());
}
    