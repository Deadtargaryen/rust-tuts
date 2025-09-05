fn main() {
    
    let mut v1: i32 = 100;

    let f1: f32 = 3.141;

    let initial: char = 'T';

    v1 = v1 + 100;

    let mut v1:i32 = 300;

{
    v1 = 50;
    println!("value v1 inside the scope {}", v1);
}

    println!("value of v1 outside the scope: {}", v1);


    println!("i16 max: {}",i16::MAX);
}
