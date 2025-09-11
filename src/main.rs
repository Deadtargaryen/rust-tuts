
fn main() {

    let mut a = 4;



    unsafe {

        let p = &a as *const i32;

        println!("pointer value: {:x?}", *p);

    }


    println!("{}", a)
}
    