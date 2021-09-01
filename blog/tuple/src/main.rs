fn main() {
    // Tuple is a primitive type, a collection type, a sequence and it is heterogeneous
    let tuple: (&str, bool, u8) = ("hello world", true, 220);
    println!("{:?}", tuple);
    // Tuple are accessed by their index:
    println!("{:?}\n{:?}\n{:?}", tuple.0, tuple.1, tuple.2);

    // Tuples can be 'destructured' (or unpacked):
    let (a, b, c) = tuple;
    dbg!(a, b, c);
    // empty tuple '()', named 'unit', used to express 'nothing'
    ();
    dbg!(());
    // We can use a tupe to return multiple values from a function:
    fn some_operation(input: i32) -> (&'static str, bool, i32) {
        if input < 10 {
            return ("not good", false, 1);
        } else {
            return ("good", true, 0);
        }
    }

    let result_a = some_operation(1);
    println!("{:?}", result_a);
    let (msg, value, exit_code) = some_operation(11);
    println!("{:?}", (msg, value, exit_code));

    let tup_1 = ("hello world", true, 220);
    let tup_2 = ("hello world", true, 220);
    let tup_3 = ("hello", false, 220);
    assert_eq!(tup_1, tup_2); // this is fine
                              //assert_eq!(tup_1, tup_3); // thread 'main' panicked at 'assertion failed: `(left == right)
    if tup_1 == tup_2 {
        println!("equal")
    } else {
        println!("not equal")
    }
}
