fn main() {
    // Specifying the type and length in square brackets when initializing:
    let arr: [u8; 3] = [0, 1, 2];
    // Have Rust infer the type:
    let arr_type_inferred = [0, 1, 2];
    // Create an array containing all 0's:
    let arr_all_0s: [u8; 3] = [0; 3];
    // show the entire array:
    println!(
        "\
arr: {:?}
arr_type_inferred: {:?}
arr_all_0s: {:?}",
        arr, arr_type_inferred, arr_all_0s
    );
    /*
    arr: [0, 1, 2]
    arr_ [0, 1, 2]
    arr__ n[0, 0, 0]
        */
    // print an item from the array:
    println!("array: {:?}\narray, element 1: {:?}", arr, arr[1]);
    /*
    array, element 1: 1
        */

    // initialize an array as mutable so we can change it later on:
    let mut a: [u8; 4] = [0; 4];
    println!("Before change: {:?}", a); // Before change: [0, 0, 0, 0]
    a[1] = 30;
    a[2] = 20;
    a[3] = 10;
    println!("After change: {:?}", a); // After change: [0, 30, 20, 10]

    a.sort(); // sort the array
    println!("After sorting: {:?}", a); // After sorting: [0, 10, 20, 30]

    // Check if a value is present inside an array:
    let x: u8 = 10;
    if a.contains(&x) {
        println!("a contains 10")
    }
    // Check if 10 is present inside the array in another way, by:
    //   calling iter() on the array, which returns an iterable
    //     using the any() iterator method, which is passed a closure
    //      will return true as soon as there is a match
    let any_res = a.iter().any(|v| v == &10);
    dbg!(any_res);
    // can use other operators using this approach:
    dbg!(a.iter().any(|v| v < &0));
    dbg!(a.iter().any(|v| v > &10));
    // iterate an array:
    for item in a.iter() {
        println!("{:?}", item);
    }
    /*
    0
    10
    20
    30
    */
    // Use 'std::array::IntoIter' implicitly:
    for item in a {
        println!("{:?}", item);
    }
    // iterate an array and get the index:
    for (index, item) in a.iter().enumerate() {
        println!("index: {:?} element {:?}", index, item);
    }
    /*
    index: 0 element 0
    index: 1 element 10
    index: 2 element 20
    index: 3 element 30
    */
    // iterate the array and change the value
    for item in a.iter_mut() {
        *item += 100;
    }
    dbg!(a);

    // find the position of a value in an array:
    dbg!(a.iter().position(|v| v == &120));
    dbg!(a.iter().position(|v| v == &121));
    // find the position of a value greater then 1 in the array:
    dbg!(a.iter().position(|v| v > &1));
    // iterate the array and 'derefence' the value:
    println!("{:?} is {} long", a, a.len());
    for i in 0..a.len() {
        println!("arr[{}] is {}", i, a[i]);
    }
    // Checking the memory size of an array:
    let i32_arr: [i32; 4] = [0, 1, 2, 3];
    let u8_arr: [u8; 4] = [0, 1, 2, 3];
    println!(
        "i32_arr is {} bytes\nu8_arr is {} bytes",
        std::mem::size_of_val(&i32_arr),
        std::mem::size_of_val(&u8_arr)
    );

    // copy semantics apply to arrays
    fn array_copy(mut arr: [u8; 4]) {
        for elem in arr.iter_mut() {
            *elem = 0;
        }
        println!("after changing it in the func {:?}", arr);
    }
    println!("before array_copy(): {:?}", a);
    array_copy(a);
    println!("after array_copy(): {:?}", a);

    // we can change the value of an array inside a func:
    fn array_ref(arr: &mut [u8; 4]) {
        for elem in arr.iter_mut() {
            *elem = 0;
        }
        println!("after changing it in the func: {:?}", arr);
    }

    println!("before array_ref(): {:?}", a);
    array_ref(&mut a);
    println!("after array_ref(): {:?}", a);
    //https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=1888f282dd4444f601cfaa4a0f3a42a5

    let row_1: [u8; 4] = [0; 4];
    let row_2: [u8; 4] = [0; 4];
    let row_3: [u8; 4] = [0; 4];
    let md_arr = [row_1, row_2, row_3];
    println!("md_arr: {:?}", md_arr);
}

/*
    //
    for item in IntoIterator::into_iter(arr).enumerate() {
        let (i, x): (usize, u8) = item;
        println!("array[{}] = {}", i, x);
    }
*/
