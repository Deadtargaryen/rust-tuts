fn main() {

    // program.exe 20
    let l:[i32;100] = [0; 100]; //array of 100 integers initialized to 0

    let mut v1:Vec<i32> = vec![65, 66, 67, 68, 69]; //vector of integers
    v1.push(100); //adding element to vector

    let temp = v1.pop().unwrap(); //removing last element from vector
    // println!("{}", temp);

    let v2:Vec<i32> = Vec::new(); //empty vector of integers

    let d = v1.iter().map(|x| {x * 2 }).collect::<Vec<i32>>();
    // println!("{:#?}", d);


    let t = (0..100).collect::<Vec<i32>>();
    // println!("{:#?}", t);

    // for i in v1.clone().into_iter(){
    //     println!("{}", i);
    // }

    // let t= l.iter().map(|x| x + 200).collect::<Vec<i32>>();
    // println!("{:#?}", t);

    // for i in l.into_iter(){
        // *i =*i + 100 ;
    //     println!("{}", i);
    // }


    // println!("{:?}", l.contains(&1)); //gives boolean value true or false
    }