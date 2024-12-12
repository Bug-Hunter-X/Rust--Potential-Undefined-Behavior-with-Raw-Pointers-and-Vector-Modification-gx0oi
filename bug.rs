fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    //The following line is the bug
    unsafe{ *ptr = 100;}
    println!("{:?}", v);
}