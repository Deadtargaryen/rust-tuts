use std::ffi::c_void;

fn main() {

    let mut a = 4;

    let mut b = [5,6,7,8];


    unsafe {

        let p = &mut b as *mut i32; // or b.as_mut_ptr()

        let temp = std::ptr::read((p as usize+12) as *mut i32);


        let q = b.as_mut_ptr() as *mut c_void;

        /*let p = &mut a as *mut i32;

        println!("pointer value: {:x?}", p);

        println!("address of a: {:x?}", std::ptr::addr_of!(a));

        // *p = *p + 10; 


        std::ptr::write(p as *mut i32, 100);

        let temp = std::ptr::read(p as *const i32);
        
        println!("{}", temp)*/

        println!("{}", temp);
    }


    // println!("{}", a)
}
    