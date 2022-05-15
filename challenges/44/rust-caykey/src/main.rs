fn main() {
    for i in 1..=100 {
        match (i % 5 == 0, i % 3 == 0) {
            (true, true) => println!("FizzBuzz"),
            (false, true) => println!("Fizz"),
            (true, false) => println!("Buzz"),
            _ => println!("{i}")
        }
    }
}