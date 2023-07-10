fn main() {
    // new - создаёт новый вектор
    let mut list: Vec<i32> = Vec::new();
    println!("{:?}", list);

    // capacity - создаёт вектор с заранее заданным размером
    let mut list: Vec<i32> = Vec::with_capacity(10);
    println!("{}", list.capacity());

    // push - добавляет значение в вектор
    list.push(5);
    list.push(10);
    list.push(15);
    list.push(20);
    list.push(25);
    list.push(30);
    list.push(35);
    println!("{:?}", list);

    // pop - возвращает последний элемент из вектора, удаляя его из вектора
    let pop = list.pop();
    println!("{:?}", pop);

    // remove - удаляет элемент по заданному индексу
    let remove = list.remove(2);
    println!("{:?}", remove);
    println!("{:?}", list);

    // get - выводит элемент по указанному индексу
    match list.get(0) {
        Some(el) => {
            println!("Элемент с индексом 0 является: {}", el);
        },
        None => {
            println!("Элемент с таким индексом отсутствует в списке!")
        },

    }

    // resize - изменяет размер вектора
    let resize = list.resize(6, 40);
    println!("{:?}", list)

}