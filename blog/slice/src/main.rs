fn main() {
    // block that defines an array and then creates 2 slices out of those arrays.
    {
        // defining an array:
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        // taking slices of the array:
        let entire_array = &array[..]; // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        let first_half = &array[0..5]; // [0, 1, 2, 3, 4]
        let second_half = &array[5..10]; // [5, 6, 7, 8, 9]
        println!(
            "{:?} {:?} {:?} {:?}",
            array, entire_array, first_half, second_half
        );
        dbg!(entire_array.len()); //10
        dbg!(second_half.len());
        dbg!(&array);
        let middle = &array[3..7];
        dbg!(middle);
        dbg!(middle.len());
        // let non_reference_slice = array[..]; // Does not work as the size is not know at compile time.
    }
    // block that shows the index of slices compared to the indexes of the backing array
    {
        // Define a mutable array.
        // Pass a slice of the array to a function and have the function
        //  change some value in the slice.
        // After that, check the value of the array.
        fn take_and_change_slice(slice: &mut [i32]) {
            println!("{:?}", slice);
            slice[1] = 90;
            println!("{:?}", slice);
        }
        let mut new_array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        take_and_change_slice(&mut new_array[8..]);
        dbg!(new_array);
        let slice = &mut new_array[8..];
        dbg!(slice);

        // Define an array and pass a slice to a function.
        // Iterate the slice using enumerate and print the index as well as the value.
        let u8_array: [u8; 4] = [0, 1, 2, 3];
        fn iterate_slice(slice: &[u8]) {
            for (index, item) in slice.iter().enumerate() {
                println!("index: {:?} element {:?}", index, item);
            }
        }
        // pass in the entire array:
        iterate_slice(&u8_array[..]);
        // pass in half the array:
        iterate_slice(&u8_array[2..]);
    }

    {
        //Block that defines an array and changes it in three differen ways:
        //  - passing it to a function, illustrating copy semantics
        //  - passing the array by reference
        //  - passing a slice of the array
        //
        // reference to an array is a thin pointer and a slice is a fat pointer
        // https://stackoverflow.com/questions/57754901/what-is-a-fat-pointer
        let mut array: [u8; 4] = [0, 1, 2, 3];
        fn copy_array(mut arr: [u8; 4]) {
            for elem in arr.iter_mut() {
                *elem = 0;
            }
            println!("array value inside the func: {:?}", arr);
        }
        println!("Before 'change_array': {:?}", array);
        copy_array(array);
        println!("After 'change_array': {:?}", array);

        fn reference_array(arr: &mut [u8; 4]) {
            for elem in arr.iter_mut() {
                *elem = 0;
            }
            println!("array value inside the func: {:?}", arr);
        }
        println!("Before 'reference_array': {:?}", array);
        reference_array(&mut array);
        println!("After 'reference_array': {:?}", array);

        fn slice_array(slice: &mut [u8]) {
            for elem in slice.iter_mut() {
                *elem = 1;
            }
            println!("array value inside the func: {:?}", slice);
        }
        println!("Before 'slice_array': {:?}", array);
        slice_array(&mut array[2..]);
        println!("After 'slice_array': {:?}", array);
    }

    // common slice operations:
    {
        let mut array: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];
        let slice = &mut array[0..5]; // [ 0, 1, 2, 3, 4 ]
                                      //let array_slice = &array[..]; // [ 0, 1, 2, 3, 4, 5, 6 ]
                                      //let array_slice = &array[0..3]; // [ 0, 1, 2 ]
                                      //let array_slice = &array[..3]; // [ 0, 1, 2 ]
                                      //let array_slice = &array[2..4]; // [ 2, 3 ]
                                      //let array_slice = &array[2..]; // [ 2, 3, 4, 5, 6 ]

        slice.len(); // 5
        slice[1]; // 1
                  //slice[100];
                  //array_slice[100]; // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100'
        slice.get(2); // Some(2)
        slice.get(100); // None
        for (index, item) in slice.iter().enumerate() {
            println!("index: {:?} element {:?}", index, item);
        }
        /*
        index: 0 element 0
        index: 1 element 1
        index: 2 element 2
        index: 3 element 3
        index: 4 element 4
        */
        slice.iter().position(|v| v == &120); // None
        slice.iter().position(|v| v == &4); // Some(4)
        slice[0] = 100;
        dbg!(slice); // [100, 1, 2, 3, 4]
        dbg!(array); // [100, 1, 2, 3, 4, 5, 6]
    }
    // block to show a slice can be taken from different types:
    {
        let array: [i32; 4] = [0, 1, 2, 3];
        let array_slice = &array[..2]; //[0, 1]
        let vector = vec![1, 2, 3, 4];
        let vector_slice = &vector[0..2]; // [1, 2]
        let string = String::from("string slice");
        let string_slice = &string[0..6]; // "string"

        println!("{:?} {:?} {:?}", array_slice, vector_slice, string_slice);
        fn return_second(n: &[i32]) {
            println!("{}", n[1]);
        }
        return_second(array_slice); // 1
        return_second(vector_slice); // 2

        fn return_second_char(n: &str) {
            println!("{:?}", n.chars().nth(1));
        }
        return_second_char(string_slice);
    }
    // block to show iterating a string slice:
    {
        let string = String::from("Rust is 😍");
        let string_slice = &string[..];
        fn return_second_char(n: &str) {
            println!("{:?}", n.chars().nth(1));
        }
        return_second_char(string_slice); // Some('u')

        for c in string_slice.chars() {
            println!("{}", c)
        }
        for (i, c) in string_slice.chars().enumerate() {
            println!("{} {}", i, c)
        }
    }
    // Block to illustrate fat and thin pointer
    {
        use std::mem;
        let xs: [i32; 5] = [1, 2, 3, 4, 5];
        let ys: [i32; 500] = [0; 500];
        let mut ys2: [i32; 500] = [0; 500];

        let xs_pointer = &xs;
        let slice = &xs[..]; //slice of whole array
        let ys_pointer = &ys;
        let middle_of_array_slice = &mut ys2[250..253]; //slice of first 3 elements
        let start_of_array_slice = &ys[0..3];

        // middle_of_array_slice[0] = 100;
        // middle_of_array_slice[1] = 200;
        // middle_of_array_slice[2] = 300;
        // println!("middle_of_array_slice: {:#?}", middle_of_array_slice);

        println!("number of elements in xs array: {}", xs.len());
        println!("number of elements in xs slice: {}", slice.len());
        println!(
            "number of elements in middle of array slice : {}",
            middle_of_array_slice.len()
        );
        println!("\n");

        println!("ys pointer address: {:p}", ys_pointer);
        println!("start of array slice address: {:p}", start_of_array_slice);
        println!("middle of array slice address: {:p}", middle_of_array_slice);
        println!("\n");

        // Array pointer just holds the address to the start of the array
        // no lenght is contained within the pointer.
        println!(
            "array pointer occupies {} bytes",
            mem::size_of_val(&xs_pointer)
        );

        // Slices are fat pointers, it contains the address of the element at the start
        // the slice. They also hold the lenght of the slice.
        // You can see this by the size difference between a slice and
        // an array pointer.
        // Also a slice doesn't need to have its size known at compile time
        // you can see that in the slice_array function. We just know its going to be a
        // pointer to a array of type i32
        println!("array slice occupies {} bytes", mem::size_of_val(&slice));
        println!(
            "array m slice occupies {} bytes",
            mem::size_of_val(&middle_of_array_slice)
        );

        println!("\n");
        slice_array(middle_of_array_slice);
        // array afterward
        println!("Surrounding values {:#?}", &ys2[249..254]);
        fn slice_array(slice: &mut [i32]) {
            for elem in slice.iter_mut() {
                *elem = 1;
            }
            println!("array value inside the func: {:?}", slice);
        }
    }
    // Block to illustrate fat and thin pointer
    {
        use std::mem;
        let array: [i32; 500] = [0; 500];
        let slice = &array[..];
        let array_pointer = &array;
        let slice_pointer = &slice;
        let start_of_array_slice = &array[0];

        println!("--------------------------------------------");
        println!("array_pointer address: {:p}", array_pointer);
        println!("slice_pointer address: {:p}", slice_pointer);
        println!("start_of_array_slice address: {:p}", start_of_array_slice);
        println!("slice occupies {} bytes", mem::size_of_val(&slice));
        println!(
            "array_pointer occupies {} bytes",
            mem::size_of_val(&array_pointer)
        );
        println!("array occupies {} bytes", mem::size_of_val(&array));
        println!("--------------------------------------------");
    }
    {
        let array: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let slice1 = &array[5..10];
        let slice2 = &array[3..7];
        dbg!(array);
        dbg!(slice1);
        dbg!(slice2);
    }
}
