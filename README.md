# 100_days_of_rust

It's my challenge!!

This is my hobby.

------------------------------

Hey, It's been ages since I last code this repo must be like 4 or 5 years already.
Alright, I think it's time for a fresh start.
Hoping this time it'll acutally work out.

This time, I'm planning to learn Rust gonna follow this guide: 
- https://doc.rust-lang.org/book/
- https://rust-lang.github.io/fls/
- https://yevh.github.io/rust-security-handbook/ [security handbook]
- https://thinknetcompany.github.io/learnrust/
- https://rust-exercises.com/100-exercises/01_intro/00_welcome.html
- https://www.youtube.com/watch?v=GVCR8b_33zo | Rust Programming by CodeBangkok EP.1
- https://www.youtube.com/watch?v=k9ZzKG8fdN8 | Rust Programming by CodeBangkok EP.2
- https://www.youtube.com/watch?v=MZRlVMoef94 | Rust Programming by CodeBangkok EP.3 HTTP Server

------------------------------

 - https://crates.io/
    - The rust community's crats registry
    - look like pypi.org of python

 - cargo new {project_name}
 
 - cargo init 
    - Create a new Cargo package in an existing directory

 - cargo add {denpencies}

 - cargo build
    - when you add new dependencies
    - if haven't anything changed, so Cargo knows it can reuse what it has already downloaded and complied for those.

 - cargo run // execute programe

 - cargo update 
    - when you do want to update a crate

 - cargo doc --open
    - build documentation provided by all your dependencies locally and open it in your browser

------------------------------

## Noted:

- ### Ownership Rules
   - Each value in Rust has a variable that's called its owner.
   - There can only be on owner at a time.
   - When the owner goes out of scope, the value will be dropped.

- ### Stack
   - Region of the process memory that stores variable created by each function.
   - Every function call a new stack frame is allocated on top of the current one.
   - Size of every variable on the stack has to be known at compile time.
   - When a function exits it's stack frame is released.

- ### Heap
   - Region of the process memory that is NOT automatically managed.
   - It has no size restrictions.
   - It's accessible by any function, anywhere in the program.
   - Heap allocations are expensive and we should aviod them when possible.
------------------------------

<B>What's coming up next:</B>
- https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values
