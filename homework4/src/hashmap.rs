use std::collections::HashMap;

fn main() {

    let mut map: HashMap<&str, u32> = HashMap::new();

    let human1 = "Дима";
    let human2 = "Соня";
    let human3 = "Серёжа";
    let human4 = "Женя";

    // insert
    map.insert(human1, 19);
    map.insert(human2, 18);
    map.insert(human3, 27);
    map.insert(human4, 18);
    println!("{map:#?}");

    // get
    let age = map.get(&human2);
    match age {
        Some(value) => {
            println!("Возраст Сони: {}", value);
        },
        None => {
            println!(":/");
        }
    }

    // remove
    map.remove("Дима");
    println!("{map:#?}");
}

