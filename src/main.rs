fn main() {

    let a = 100;
    let b = 50;
    let c = 25;

    let mut d: i32 = 0;

    d += 100; // d = d + 100
    d -= 50;  // d = d - 50
    d *= 100;  // d = d * 100
    d /= 20;  // d = d / 20


//operations
    println!("{}",a|b);
    println!("{}",a&b);
    println!("{}",a^b);
    println!("{}",!b);
    println!("{}",a%b);

    /*for i in 1..=100{
        println!("i : {}",i);
    }*/

    let mut i = 0;
    while i<101{
        print!("{} ",i);
        i+=2;
    }


}
