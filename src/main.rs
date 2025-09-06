fn main() {

    // program.exe 20
    let mut l:[i32;5] = [5,6,7,8,9];

    for i in l.into_iter(){
        // *i =*i + 100 ;
        println!("{}", i);
    }


    println!("{:?}", l.contains(&8)); //gives boolean value true or false
    }