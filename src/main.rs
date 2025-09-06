fn main() {

    let l:[i32;5] = [5,6,7,8,9];

    println!("{:#?}", l.as_ptr());

    unsafe{
    let temp = std::ptr::read((l.as_ptr() as isize + (std::mem::size_of::<i32>()*3) as isize) as *const u8);
    println!("{}", temp);
    }
}