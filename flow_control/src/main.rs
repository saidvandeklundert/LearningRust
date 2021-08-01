fn main() {
    println!("Hello, world!");
    /*
     */

    /*
     */
    let item = 32;
    match item {
        0 => {
            dbg!("0");
        }
        10..=20 => {
            dbg!("between 10 and 20");
        }
        40 | 80 => {
            dbg!("40 OR 80");
        }
        _ => {
            dbg!("all else");
        }
    }
    /*
    For loop
    */
    let array: [u8; 3] = [1, 2, 3];
    for i in array {
        print!("{} ", i)
    }
}
