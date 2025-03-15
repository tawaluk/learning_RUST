fn main() {
    println!("Структуры:");

    println!("Пример. Простейший:");
    struct_test();
    println!("Пример с частичной передачей полей:");
    let mut user1 = struct_test_2(22);
    println!("Пример с передачей структуры:");
    struct_test_3(user1);
    println!("Пример с изменением структуры:");
    let user2 = struct_test_2(60);
    struct_test_4(user2);

}

// Обьявление структуры. Область видимости - весь модуль
struct User {
    name: String,  // не &str, потому что так надежнее и независимо.
    age: i32,
    male: bool
}

fn struct_test() {
    let user = User {
        name: String::from("Петя"),
        age: 15,
        male: true
    };

    println!("Имя: {} Возраст: {}", user.name, user.age);
}

fn struct_test_2(age: i32) -> User {
    let user1 = User {
        name: String::from("Вася"),
        age,  // Эта запись аналогична записи: age: age, потому что имя и значение имеют одинаковое имя.
        male: true
    };

    println!("Имя: {} Возраст: {}", user1.name, user1.age);
    
    user1

}

fn struct_test_3(user: User) {
    println!("Имя: {} Возраст: {}", user.name, user.age);
}

fn struct_test_4(mut user: User) {
    user.age = 10000;
    println!("Имя: {} Возраст: {}", user.name, user.age);
}