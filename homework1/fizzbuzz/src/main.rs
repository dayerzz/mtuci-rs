// first version
fn main() {
    for i in 1..=100 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        }
        else if i % 5 == 0 {
            println!("Buzz");
        }
        else if i % 3 == 0 {
            println!("Fizz");
        }
        else {
            println!("{i}");
        }
        //fizzbuzz() // to run the second version of the code
    }
}

// second version
fn fizzbuzz() {
    for i in 1..=100 {
        match(i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (_, 0) => println!("Buzz"),
            (0, _) => println!("Fizz"),
            _ => println!("{i}")
        }
    }
}