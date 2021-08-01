fn simple_example() -> String {
    println!("Functions should use snake_case");
    let x = "Functions can return stuff";
    format!("{}", x)
    // the previous is the same as putting return in front of it and ; behind it
    //return format!("{}", x);
}
fn main() {
    println!("Hello, world!");
    // define x to store the func return and print it out later on:
    let x = simple_example();
    println!("{}", x);

    let width = 4;
    let height = 7;
    let depth = 10;
    {
        let area = area_of(width, height);
        println!("Area is {}", area);
    }

    println!("Volume is {}", volume(width, height, depth));
}

fn area_of(x: i32, y: i32) -> i32 {
    x * y
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}
