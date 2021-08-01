fn main() {
    // the arg to add() can be any type that implements std::ops::Add
    fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
        i + j
    }
    let aa: u8 = 10;
    let ab: u8 = 30;
    add(aa, ab);
    let ba: i32 = 10;
    let bb: i32 = 30;
    add(ba, bb);

    let float_addition = add(1.2, 2.65);
}
