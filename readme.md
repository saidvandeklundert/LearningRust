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





[book](https://doc.rust-lang.org/book/)
[std modules](https://doc.rust-lang.org/std/#modules)



Hoover over func, hold CTRL and lmb. Then you are taken to the source code:
![Go to source](https://github.com/saidvandeklundert/LearningRust/blob/master/img/view_source_1.png)
![Go to source](https://github.com/saidvandeklundert/LearningRust/blob/master/img/view_source_2.png)

