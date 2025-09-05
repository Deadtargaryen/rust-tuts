fn main() {
    let a = 10;
    let b = 50;
    let c = 25;

    let mut d: i32 = 0;

    d += 100; // d = d + 100
    d -= 50; // d = d - 50
    d *= 100; // d = d * 100
    d /= 20; // d = d / 20

    //operations
    println!("{}", a > b);
    println!("{}", a < b);
    println!("{}", a == b);
    println!("{}", a >= b);
    println!("{}", a % b);

    /*for i in 1..=100{
            println!("i : {}",i);
        }

        let mut i = 0;
        while i<101{
            println!("{} ",i);
            i+=2;
        }

        if a>b{
            println!("a is greater than b");
        } else if a==b {
            println!("a is equal to b");
        } else{
            println!("a is lesser than b")
        }
    */

    let mut i =0;
    loop {

        if i==1000{
            break;
        }

        println!("{}", i);
        i+=1;

    }
}
