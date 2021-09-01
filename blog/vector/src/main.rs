fn main() {
    println!("Samples from http://saidvandeklundert.net/learn/2021-08-15-rust-vector/");
    // Block with basic vector operations
    {
        let mut vector: Vec<i32> = Vec::new();
        println!("{:?}", vector); // []
        vector.push(0); // [0]
        println!("{:?}", vector);
        vector.push(1); // [0, 1]
        println!("{:?}", vector);
        vector.push(2); // [0, 1, 2]
                        //
                        //
                        //
        println!("{:?}", vector);
        println!("{:?}", vector.pop()); // Some(2)
        println!("{:?}", vector); // [0, 1]
        println!("{:?}", vector[1]); // 1
        println!("{:?}", vector.get(1)); // Some(1)
        println!("{:?}", vector.get(100)); // None
        let mut vec = vec![2, 2, 3, 4, 5];
        println!("{:?}", vec); // [2, 2, 3, 4, 5]
        vec.remove(0);
        println!("{:?}", vec); // [2, 3, 4, 5]
        vector.append(&mut vec);
        println!("{:?}", vec); // []
        println!("{:?}", vector); // [0, 1, 2, 3, 4, 5]
        println!("{:?}", vec.is_empty()); // true
        println!("{:?}", vector.is_empty()); // false
        println!("{:?}", vector.len()); // 6
        let vecter_iterator = vector.iter();
        for elem in vecter_iterator {
            println!("{}", elem);
            // elem += 1; > will not work, need iter_mut()
        }
        let vecter_iterator_m = vector.iter_mut();
        for elem in vecter_iterator_m {
            *elem = *elem * 2;
        }
        println!("{:?}", vector); // [0, 2, 4, 6, 8, 10]

        println!("{:?}", vector.contains(&200)); // false
        println!("{:?}", vector.contains(&2)); // true
        vector.insert(2, 1);
        println!("{:?}", vector); // [0, 2, 1, 4, 6, 8, 10]
        println!("{:?}", vector.sort());
        println!("{:?}", vector); // [0, 1, 2, 4, 6, 8, 10]
        println!("{:?}", vector.binary_search(&4)); // Ok(3)
        println!("{:?}", vector.binary_search(&400)); // Err(7)
        vector.resize(10, 0);
        println!("{:?}", vector); // [0, 1, 2, 4, 6, 8, 10, 0, 0, 0]
        vector.resize(2, 0);
        println!("{:?}", vector); // [0, 1]
        dbg!(vector);
    }
    {
        #[derive(Debug)]
        enum ManyAsOne {
            String(String),
            I32(i32),
            F64(f64),
        }
        let vec = vec![
            ManyAsOne::I32(65444),
            ManyAsOne::String(String::from("Hello world.")),
            ManyAsOne::String(String::from("Foo bar.")),
            ManyAsOne::F64(3.14159265),
            ManyAsOne::I32(1984),
        ];
        for elem in vec {
            println!("{:?}", elem);
            match elem {
                ManyAsOne::I32(value) => println!("value: {}", value),
                ManyAsOne::String(value) => println!("value: {}", value),
                ManyAsOne::F64(value) => println!("value: {}", value),
            }
        }
    }
    {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
            interests: Vec<String>,
        }

        let mut marie = Person {
            name: String::from("Marie"),
            age: 31,
            interests: vec![
                String::from("Rust"),
                String::from("Python"),
                String::from("History"),
            ],
        };
        marie.interests.push(String::from("Astronomy"));
        println!("{:?}", marie);
    }
    {
        let mut vec: Vec<i32> = Vec::with_capacity(6);
        vec.push(1);
        vec.push(2);
        println!("--------------------------------------------");
        println!("Pointer to vec: {:p}", &vec);
        println!("Pointer to vec: {:p}", &vec[0]);
        println!("Pointer to vec: {:p}", &vec[1]);
        println!("Values inside vec: {:?}", vec);
        println!("Length of the vec: {:?}", vec.len());
        println!("Capacity of the vec: {:?}", vec.capacity());
        println!("--------------------------------------------");
        let mut vector = vec![3, 4, 5, 6, 7];
        vec.append(&mut vector);
        println!("--------------------------------------------");
        println!("Length of the vec: {:?}", vec.len());
        println!("Capacity of the vec: {:?}", vec.capacity());
        println!("--------------------------------------------");
    }
    {
        use std::collections::HashMap;
        let mut job_results: HashMap<String, Vec<i32>> = HashMap::new();
        job_results.insert(String::from("1"), vec![3, 2, 2, 2, 2]);
        job_results.insert(String::from("2"), vec![2, 3, 2, 2, 2]);
        job_results.insert(String::from("3"), vec![2, 2, 3, 2, 2]);
        job_results.insert(String::from("4"), vec![2, 2, 2, 3, 5]);
        println!("Length of the vec: {:?}", job_results);
    }
}
