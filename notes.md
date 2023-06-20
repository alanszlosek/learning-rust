Things to research and use

* #![allow( BLA )] stuff and possibilities
* #[repr(C)]
* tuple struct ... think it's a way to declare a struct using array syntax so each element is same size and type
* struct update syntax: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax
* turbofish and Template syntaxes
* declaring vars with uninitialized memory
* raw pointer operations: https://doc.rust-lang.org/std/primitive.pointer.html#methods
    - write writes data without reading it (so destructor on object that's being overwritten doesn't get called)
* declaring things on the heap so can pass around
* lifetimes

Practice

* range loops
* unchecked pointer arithmatic: *bodies.add(i)
* checked pointer arithmatic: bodies[i] (i believe)
