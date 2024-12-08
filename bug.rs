fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    //The following line is incorrect and can lead to undefined behavior
    unsafe {*ptr = 100};
    println!("{:?}", v);
}