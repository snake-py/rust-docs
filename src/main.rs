mod games;
use games::game::NumberGuessingGame;

fn main() {
    NumberGuessingGame::run();
}

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
