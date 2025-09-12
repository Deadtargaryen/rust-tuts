
fn main() {

    let mut a = 4;



    unsafe {

        let p = &mut a as *mut i32;

        println!("pointer value: {:x?}", p);

        println!("address of a: {:x?}", std::ptr::addr_of!(a));

        *p = *p + 10; 

    }


    println!("{}", a)
}
    