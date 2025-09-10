fn main() {
    let mut s = String::from("tech69");

    let s2 = &s;
    let s3 = &s;

    s.push_str( "hello");

    //as long as a value is borrowed and in scope, you cannot change those values
    println!("{}", s);
    println!("{}", s2);

}
    