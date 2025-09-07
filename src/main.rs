fn main() {

    // program.exe 20
    let l:[i32;100] = [0; 100]; //array of 100 integers initialized to 0
    let mut name:Vec<char> = vec!['a', 'b', 'c', 'd', 'e']; //vector of characters
    let mut v1:Vec<i32> = vec![65, 66, 67, 68, 69]; //vector of integers
    v1.push(100); //adding element to vector

    let temp = v1.pop().unwrap(); //removing last element from vector
    // println!("{}", temp);

    let mut v2:Vec<i32> = Vec::new(); //empty vector of integers
    v2.push(99);
    v2.push(99);
    v2.push(99);
    v2.push(99);
    v2.push(99);

    let d = v1.iter().map(|x| {x * 2 }).collect::<Vec<i32>>();
    // println!("{:#?}", d);


    let t = (0..100).collect::<Vec<i32>>();
    // println!("{:#?}", t);

    let mut v3:Vec<Vec<i32>> = Vec::new();
    v3.push(v1);
    v3.push(v2);
    println!("{:#?}", v3);
    }