use std::io;
fn main() {
    //    let guess: u32 = "42".parse().expect("Not a number!");
    // let guess = "42".parse().expect("Not a number!");
    //   let a = [1, 2, 3, 4, 5];

    //     println!("Please enter an array index.");

    //     let mut index = String::new();

    //     io::stdin()
    //         .read_line(&mut index)
    //         .expect("Failed to read line");

    //     let index: usize = index
    //         .trim()
    //         .parse()
    //         .expect("Index entered was not a number");

    //     let element = a[index];

    //     println!("The value of the element at index {index} is: {element}");

    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // println!("The value of number is: {number}");

    //   let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("the value is: {element}");
    // }
    // let mut s = String::from("hello");

    // s.push_str(", Ajidokwu!"); // push_str() appends a literal to a String

    // println!("{s}"); // This will print `hello, world!`

    let s1 = String::from("hello");
    let _s2 = s1.clone();

    println!("{s1}, world!");
}
