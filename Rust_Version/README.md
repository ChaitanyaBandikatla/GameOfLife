# GameOfLife in Rust
This folder contains working code for Conway's game of life in Rust language.

To build and run the game of life on your machine, follow the below steps:

 - Firstly, you need to install rust from [here](https://www.rust-lang.org/tools/install). This will also install [cargo](https://github.com/rust-lang/cargo), so it is not required to install it separately.
 - After that, you can build the code using `cargo build`.
 - Then, the code can be run using `cargo run`.
 - You should be able to enter your desired inputs for the parameters - number of rows, number of columns, number of live cells to start with, and number of steps/iterations.
 - The code would print the board after every iteration!
 
Alternatively if you want to make your life simpler, you can use this online [playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=3f20ae37c8fe6df22c60ba7821113d76).

Please note that the rust playground does not support standard input. So, if you would like to change the parameter values, you need to change them directly in the code in `main` function. The part of the code responsible for inputs has been commented out.
 
# Debugging Rust

The cargo build and run reports the results in comprehensive detail, making the region of error very straightforward to track and debug.

For using print statements in debugging, the syntax for the rust programming language is as follows:

`println!("YOUR STATEMENT HERE");` prints `YOUR STATEMENT HERE`
