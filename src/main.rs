use std::ffi::c_void;

fn main() {

    let mut a = 4;

    let mut b = vec![5,6,7,8];


    unsafe {

        let p = &b  as *const Vec<i32>;

        println!("address of a: {:x?}", std::ptr::addr_of!(b));
        
        let q = b.as_mut_ptr();

        //reading len & capacity of vectors
        let temp = std::ptr::read((p as usize+8) as *mut i32);
        println!("{:x?}", temp);
        


    }


    // println!("{}", a)
}
    