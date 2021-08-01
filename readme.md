### Things about Rust

Rust is a general-purpose language.

Has the ability to prevent invalid data access at compile time. This eliminates a category of bugs and makes the programs written in Rust memory safe. Since it is the compiler handling it, there is no runtime cost.

Rust aims:
- safety: reason for the creed 'fearless concurrency'. Rust aims to provent the following:
  - dangling pointers
  - data races
  - buffer overlow (access 4th element of 2 element array)
  - iterator invalidation (issue caused by iterating over something changed halfway)
- productivity: ergonomic features in the language and cargo.
- control


Big Rust features:
- performance: no garbage collector or runtime environment. A 'big' language instead with a compiler that does a lot of checking.
- concurrency
- memory efficienct


Downsides to Rust:
- cyclic data structures (like linked lists) are very hard
- compile time is slow
- strictness, to the point where it can be annoying
- huuuuuuuuuge language

### Rust terms and terminology

Borrow:

Move:

Trait: collection of methods implemented on a type.

Unit: an empty tuple, written like so: `()`.

Zero cost abstractions: features that Rust offer do not impose a runtime cost.

Crate: a Rust package.

No Rust 2.0: The promise is that there will never be a Rust 2.0 as Rust will also be backwards compatible.

Rust prelude: the things that Rust imports for you automatically. Described [here](https://doc.rust-lang.org/std/prelude/index.html).

Predicate: a condition, for instance in a for loop. Something that an expression evaluates to, like `true` or `false`.

Reference: a value that stands in place for another value or a pointer to another value. There is a reference operator (`&`) and a dereference operator (`*`). The reference operator refers to the value. The dereference operator allows you to work with the value that the reference is pointing to.

Lifetime annotations: used to declare the intent and signal the lifetime of a value. Most lifetime checks are unaided and the compiler takes care of them automatically as they are infered from the source code.

The `'static` lifetime: hard-coded read-only section of an executable program that is read-only during execution. The `&str` is short for `&'static str`.

Generic types: 

### Statement vs expression

A statement is not an expression. In Rust, statements appear in three places:
- binding a name to a value using `=`
- expressions delimited by `;`
- type declarations, which includes data types created with the `struct` and `enum` keywords as well functions (defined using `fn`)


### Ownership rules

1. Each value in Rust is owned by a variable.

2. When the owner goes out of scope, the value will be deallocated.

3. There can only be ONE owner at a time.

### The stack
- region of the process memory that stores variables created by each function
- for every function call, a new stack frame is allocated on top of the current one
- the size of every variable on the stack has to be known at compile time
- when a function exits it's stack frame is released


Stack is of limited size. If we exceed the stack, through endless recursion for instance, the program will crash with a stack overflow error.


### The heap

- region of the process memory that is NOT automatically managed (needs to be allocated and freed)
- no size restrictions
- accessible by any function, anywhere in the program ( variables on the stack are accessible only by the function that allocated them)
- a lot more expensive when compared to the stack


### Smart pointer

A smart pointer is just a wrapper around a raw pointer adding additional capabilities to it. There are different types of smart pointers. The most common one will make sure to free the memory it points to when the pointer goes out of scope.


### Borrowing and moving

When variables are re-assigned, it matters whether or not the value that the variable points to is created on the heap or on the stack.

Borrowing: passing references as parameters.

At any given time inside a scope, we can have :
- one mutable borrow, OR
- many immutable borrows

This restrictions imposed by the borrow checker prevents data races at compile time. No performance price is paid during run time.


### Gdb tips

```
cd xxx/target
gdb cli_mars.exe
b calculate_weight_on_mars      ! set breakpoint
r                               ! run
info locals
info args
```


### Install

Centos:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
yum -y install gcc
```


### Cargo & tools

Build system and package manager.

```
cd %TMP%
cargo new                       // create new rust project
cargo rustc --release           // build for real
cargo run --release             // run release build
cargo run -q --release          // run release build extra quiet
cargo run -v                    // cargo run but with additional info
cargo add num                   // let cargo add the num crate to the project

cargo install cargo-edit        // add usefull cargo tool
cargo add nums                  // using cargo-edit, update the toml in the package to include nums
cargo doc                       // generate the doc for a crate locally
cargo doc --open                // generate and open the documentation in the browser
```

Rustup manages your Rust installation(s).


---

[book](https://doc.rust-lang.org/book/)
[std modules](https://doc.rust-lang.org/std/#modules)



Hoover over func, hold CTRL and lmb. Then you are taken to the source code:
![Go to source](https://github.com/saidvandeklundert/LearningRust/blob/master/img/view_source_1.png)
![Go to source](https://github.com/saidvandeklundert/LearningRust/blob/master/img/view_source_2.png)

