

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














[book](https://doc.rust-lang.org/book/)
[std modules](https://doc.rust-lang.org/std/#modules)


![Go to source](https://github.com/saidvandeklundert/LearningRust/blob/main/img/view_source_1.png)
![Go to source](https://github.com/saidvandeklundert/LearningRust/blob/main/img/view_source_2.png)