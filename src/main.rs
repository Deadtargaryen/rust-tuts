fn testing () -> String{
    //dangling reference 
    let myname = String::from("lenovo");
    return myname;
}


fn main() {
    let mut s = String::from("tech69");

    let reader1 = &s;
    let reader2 = &s;

    let writer1 = &mut s;
    //let writer2 = &mut s; //cannot borrow as mutable more than once

    //testing(&mut s);

    //as long as a value is borrowed and in scope, you cannot change those values
    //consuming means taking ownership of a value i.e it is a taking a parameter and it is doing any operation to the value
    println!("{}", writer1);
    
    println!("{}", s);

}
    