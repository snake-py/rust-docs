use rand::Rng;
use std::cmp::Ordering;
use std::io;

// pub struct Dog {
//     name: String,
//     age: i32,
// }

// impl Dog {
//     pub fn bark() {
//         println!("Wuff");
//     }
//     pub fn personalized_bark(&self) {
//         let bark = "Wuff! I am ";
//         let age = " and I am ";
//         let test = &self.name;
//         let output = bark.to_owned() + test + age + &self.age.to_string();
//         println!("{}", output)
//     }
// }

// fn test_struct() {
// Dog::bark();
// let doggy = Dog {
//     name: String::from("Max"),
//     age: 18,
// };
// doggy.personalized_bark()
// }

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {secret_number}.");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // this will still crash the program
        // let guess: u32 = guess.trim().parse().expect("Please input a number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your input could not be converted to a number");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
    }
}
