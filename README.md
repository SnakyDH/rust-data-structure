# rust-data-structure
This repository is a collection of data structures and algorithms implemented in Rust. The purpose of this repository is to help me learn Rust and data structures and algorithms. I hope it can help you too.
## Presentation
[PowerPoint](https://udistritaleduco-my.sharepoint.com/:b:/g/personal/dangarciar_udistrital_edu_co/Ecw87NNcMwtEorYQv57vuv0Bsd-Px9Pqu2YsjjakNJofog?e=SmmnI4)
## Variables
### Variable Declaration
```rust
  // numbers variable
  let x:i8 = -5;
  let y:u8 = 5;
  // string variable
  let name:&str = "John";
  // boolean variable
  let is_active:bool = true;
  // char variable
  let a1:char = 'a';
  // float variable
  let f1:f32 = 5.5;
  // array variable
  let arr1:[i32; 5] = [1, 2, 3, 4, 5];
  // tuple variable
  let tup1:(i32, f64, char) = (1, 2.5, 'a');
  // multiple variable
  let (x, y, z) = tup1;
  // function variable
  let add = |x:i32, y:i32| -> i32 { x + y };
  // function variable with type
  let add:fn(i32, i32) -> i32 = |x, y| x + y;
```
## Input and Output
### Input
```rust
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();
  let input:i32 = input.trim().parse().unwrap();
```
### Output
``` rust
  println!("Hello, world!");
  println!("The value of x is: {}", x);
```
## If conditional 
``` rust
if condition {

}
```
## loop
``` rust 
  loop{
    break;
  }
  for i in 0..10 {
    println!("{}", i);
  }
```
## cargo
```rust
  use regex::Regex;
```