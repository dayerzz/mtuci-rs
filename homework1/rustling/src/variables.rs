// variables1
pub fn variables1() {
    let x = 5;
    println!("x has the value {}", x);
}

// variables2
pub fn variables2() {
    let x: u32 = 10;
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}

// variables3
pub fn variables3() {
    let x: i32 = 100;
    println!("Number {}", x)
}

// variables4
pub fn variables4() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5;
    println!("Number {}", x);
}

// variables5
pub fn variables5() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}

// variables6
pub fn variables6() {
    const NUMBER:i32 = 3;
    println!("Number {}", NUMBER);
}