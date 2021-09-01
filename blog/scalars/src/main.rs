fn main() {
    /*
    Types:
    https://doc.rust-lang.org/reference/types.html
    Scalar types:
      - booleans
      - characters
      - integers
      - floats
    */

    //Scalar types
    let ch: char = 'z';
    let b: bool = true;
    let i: i32 = -2323;
    let f: f32 = 3.4;
    println!("char: {}\nbool: {}\ni32: {}\nfloat32: {}\n", ch, b, i, f);
    // integers can be 'annotated':
    let c = 30i32;
    let d = 30_i32; // the _ is ignored by the compiler, it can help improve readability
    let e: i32 = 10_000; // underscore is ignore, used to improve readability
    println!(
        "annotated i32: {}\nanother annotated i32: {}\nand another annotated i32: {}\n",
        c, d, e
    );
    //Using the dbg! macro can also return the value to the console:
    dbg!(c);

    // Using scalars and primitive types in func is done using copy semantics:
    copy_semantics(i, ch, b, f);
    // We can still use the values after the function scope closes:
    println!("char: {}\nbool: {}\ni32: {}\nfloat32: {}\n", ch, b, i, f);

    // All other types use move-semantics:
    let marie = Person {
        name: String::from("marie"),
        age: 2,
    };
    move_semantics(marie);
    // Next line is illegal because a move happend when we passed marie to a function:
    //println!("{} is {} years old.", person.name, person.age);
}

fn copy_semantics(number: i32, character: char, boolean: bool, float: f32) {
    println!(
        "char: {}\nbool: {}\ni32: {}\nfloat32: {}\n",
        number, character, boolean, float
    );
}

fn move_semantics(person: Person) {
    println!("{} is {} years old.", person.name, person.age);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
