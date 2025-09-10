fn testing (name:String) -> String{
    return name;
}

fn main() {
    let mut s = String::from("tech69");

    let s2 = &s;
    let s3 = &s;

    s = testing(s);

    //as long as a value is borrowed and in scope, you cannot change those values
    println!("{}", s);

}
    