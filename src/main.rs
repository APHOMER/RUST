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



// fn main() {
// // https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices

//     // let s = String::from("hello world");
//     let s = String::from("APHOLABI MEXICO");
//     let arr = [1,2,3,4,5,6];

//     // let hell = &s[0..5];
//     // let wor = &s[6..11];

//     let nick = &s[..4];
//     let country = &s[9..];
//     let length = s.len();

//     println!("{}", nick);
//     println!("{}", country);
//     // println!("{}", s.length);
//     println!("{}", &s[..]);
//     println!("{}", &s[0..length]);
//     println!("{}", length);
    
//     let slice = &arr[1..4];
//     println!("{:?}", slice);

//     let eq = assert_eq!(slice, &[2, 3]);

//     println!("{:?}", eq);
    
// }




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


// STRUCT

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let user1 = User {
//         username: String::from("Aphomer"),
//         email: String::from("aaphomer@example.com"),
//         sign_in_count: 1,
//         active: true,
//     };
// }



// MUTABLE
// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("Aphomer"),
//         email: String::from("aaphomer@example.com"),
//         sign_in_count: 1,
//     };

//     user1.email = String::from("aaphomer1@example.com");
// }



// AREA OF A RECTANGLE

// fn main () {
//     let length = 20;
//     let breadth = 5;

//     println!("The area of this Carton is {} square pixels...", carton_area(length, breadth));
// }

// fn carton_area(length: u32, breadth: u32) -> u32 {
//     length * breadth
// }




///////////////////////////////////

// struct Rectangle {
//     length: u32,
//     breadth: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         length: 30,
//         breadth: 40,
//     };

//     println!( "The area of the rectangle is {} square pixels.", area(&rect1));

// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.length * rectangle.breadth
// }



// DERIVE

// #[derive(Debug)]
// struct Rectangle {
//     length: u32,
//     breadth: u32,
// }

// fn main () {
//     let rect1 = Rectangle {
//         length: 25,
//         breadth: 35,
//     };

//     // println!("RECT1 IS {rect1:?}");
//     println!("RECT1 IS {rect1:#?}");
// }



// #[derive(Debug)]
// struct Rectangle {
    // length: u32,
    // breadth: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
    // length: dbg!(30 * scale),
    // breadth: 50,
//     };

//     dbg!(&rect1);
// }


// DEFINING METHODS

// #[derive(Debug)]
// struct Rectangle {
//     length: u32,
//     breadth: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.length * self.breadth
//     }

//     fn perimeter(&self) -> u32 {
//         (self.length * 2) + (self.breadth * 2)
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         length: 20,
//         breadth: 40,
//     };

//     println!("The area of the rectancgle is {} square pixels", rect1.area());

//     println!("The perimeter of the Rectangle is {} square meters", rect1.perimeter())
// }



// #[derive(Debug)]
// struct Rectangle {
//     length: u32,
//     breadth: u32,
// }

// impl Rectangle {
//     fn length(&self) -> bool {
//         self.length > 0
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.length > other.length && self.breadth > other.breadth
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         length: 30,
//         breadth: 50,
//     };

//     if rect1.length() {
//         println!("The rectangle has a nonzero length; it is {}", rect1.length);
//     }
// }


// fn main() {
//     let rect1 = Rectangle {
//         length: 30,
//         breadth: 50,
//     };
//     let rect2 = Rectangle {
//         length: 10,
//         breadth: 40,
//     };
//     let rect3 = Rectangle {
//         length: 60,
//         breadth: 45,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
//     println!("Can rect3 hold rect1? {}", rect3.can_hold(&rect1));
//     println!("Can rect3 hold rect2? {}", rect3.can_hold(&rect2));
// }



// ENUM

// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }

//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }


// CLEARER ENUM

// fn main() {
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }

//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));
// }


// fn main() {
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }

//     let home = IpAddr::V4(127, 0, 0, 1);

//     let loopback = IpAddr::V6(String::from("::1"));
// }




// fn main() {
//     enum Message {
//         Quit,
//         Move { x: i32, y: i32 },
//         Write(String),
//         ChangeColor(i32, i32, i32),
//     }

//     impl Message {
//         fn call(&self) {
//             // method body would be defined here
//             println!("I am ENUM")
//         }
//     }

//     let m = Message::Write(String::from("hello"));
//     m.call();
// }


// EXPRESSING NULL VALUE 
fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}


