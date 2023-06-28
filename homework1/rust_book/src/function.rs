pub fn main1() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

pub fn main2() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

pub fn main3() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}


