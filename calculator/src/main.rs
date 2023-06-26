use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn main() {
    // Приветствие
    println!("Welcome!");
    println!("----------");

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();

    // Ввод первого числа
    print!("Enter the first number:");
    read(&mut num1);

    // Вывод операции
    print!("Enter the operation [+, -, *, /, %]:");
    read(&mut operator);

    // Ввод второго числа
    print!("Enter the second number:");
    read(&mut num2);

    let num1: f32 = num1.trim().parse().unwrap();
    let num2: f32 = num2.trim().parse().unwrap();
    let operator: char = operator.trim().chars().next().unwrap();

    let operators = String::from("+, -, *, /, %");

    // Вывод ошибки, если введен неверный оператор
     if !operators.contains(operator) {
        println!("An unknown operation was introduced!");
        return;
    }

    // Присваивание операций к результату и вывод паники
    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        '%' => num1 % num2,
        _ => panic!("Error")
    };

    // Вывод результата
    println!("The result of {} {} {} = {}", num1, operator, num2, result);
}
