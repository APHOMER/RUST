// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     // println!("The secret number is: {secret_number}");

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         // --snip--

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         println!("You guessed: {guess}");

//         // --snip--

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }




// println!("DATA STRUCTURE.");


// use std::io;

// fn main() {

    
//     println!("DATA STRUCTURE.");

//     let a = [1, 2, 3, 4, 5];

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
//     let double_element = a[index] * 2;

//     println!("The value of the element at index {index} is: {element}");
//     println!("The DOUBLE value of the element at index {index} is: {double_element}");
// }


// fn main () {

//     square_of_x(4)
// }

// fn square_of_x(x: i32) {
//     println!("the Square of x is {x}*{x}");
// }


// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: '{value}{unit_label}'");
// }




// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!"); // push_str() appends a literal to a String

//     s.push_str(", I am Aphomer");
//     s.push_str(", of God !");
//     println!("{s}"); // This will print `hello, world!`
// }




// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {s1}, s2 = {s2}");
// }




// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()

// }

// fn main() {
//     // let mut s = String::from("hello world");
//     let s = String::from("AFOLABI Mercy");

//     let word = first_word(&s); // word will get the value 5

//     // s.clear(); // this empties the String, making it equal to ""

//     // word still has the value 5 here, but there's no more string that
//     // we could meaningfully use the value 5 with. word is now totally invalid!

    
//     println!("{}", word);

// }



fn main() {
// https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices

    // let s = String::from("hello world");
    let s = String::from("APHOLABI MEXICO");
    let arr = [1,2,3,4,5,6];

    // let hell = &s[0..5];
    // let wor = &s[6..11];

    let nick = &s[..4];
    let country = &s[9..];
    let length = s.len();

    println!("{}", nick);
    println!("{}", country);
    // println!("{}", s.length);
    println!("{}", &s[..]);
    println!("{}", &s[0..length]);
    println!("{}", length);
    
    let slice = &arr[1..4];
    println!("{:?}", slice);

    let eq = assert_eq!(slice, &[2, 3]);

    println!("{:?}", eq);
    
}


// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
//     // println!("{}", &s[..]);
// }

// fn main() {
//     // println!("{}", &s[..]);

// }





