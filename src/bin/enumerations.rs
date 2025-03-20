fn main() {
    println!("Перечисления это типы данных, которые могут принимать одно из нескольких значений.");
    println!("Перечисления могут быть неограниченной длины, а могут быть ограничены.");
    println!("Особенно это удобно, когда необходимо передавать данные в функции и когда есть возможность не передавать ничего.");
    example();
}

#[derive(Debug)] /// для вывода в консоль. нужно, потому что перечисление является пользовательским типом данных 
                    // и не имеет реализации методов для вывода в консоль по умолчанию
enum Name {
    full_name(String),
    s_f_m(String, String, String),
    not_name(Option<String>)
}

fn example() {
    let name1 = Name::full_name("John000".to_string());
    let name2 = Name::s_f_m("Ivanov".to_string(), "Ivan".to_string(), "Ivanovich".to_string());
    let name3 = Name::not_name(None);

    println!("ФИО: {:?}", name1);
    println!("Ф_И_О: {:?}", name2);
    println!("Не ФИО: {:?}", name3);
}
