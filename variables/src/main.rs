const CAN_NEVER_CHANGE: &str = "TRULY IMMUTABLE";

fn main() {
    println!("Hello, world!");
    let string1 = "word";
    let string2: &str = "out";
    println!("The {} is {}.", string1, string2);
    /*
    Hello, world!
    The word is out.
            */
    let (words, words2) = ("word", "another word");
    println!(
        "assigned '{}' and '{}' to words and words2 in a single line",
        words, words2
    );
    let mut to_be_changed: &str = "before";
    println!(
        "Use let mut to allow variables, such as to_be_change, change from {}",
        to_be_changed
    );
    to_be_changed = "after";
    println!("into {}.", to_be_changed);
    /*
        Use let mut to allow variables, such as to_be_change, change from before
    into after.
        */
    println!("A const, such as {}, is truly immutable. Constants can be declared outside of functions and can be 'global'.
    Also, they are faster as they are inlined at compile time.", CAN_NEVER_CHANGE);
    // scopes and variables:

    let x = 10;
    {
        let y = 100;
        println!("{}, {} are in scope", x, y);
    }
    println!("{}, is in scope, y is out of scope and destroyed.", x);

    // beware the shadow

    let z = 1000;
    {
        let z = 100;
        println!("Inside the block, z is {}", z);
    }
    println!("Outside the block, z is {}\nWHoa!", z);

    // in the context of memory safety, the compiler MUST be able to ensure it can evaluate a value:
    let cypher: i32;
    if true {
        cypher = 100;
    } else {
        cypher = 200;
    }
    println!("{}", cypher)
}
