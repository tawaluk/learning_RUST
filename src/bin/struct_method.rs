fn main() {
    println!("Methods:");
    println!("
        В раст методы принято писать сразу после структуры. 
        Обьявление методов происходит в блоке impl, в котором указывается имя структуры и имя метода.
        В методе можно использовать поля структуры, которые были переданы в методе.
        Первый параметр метода всегда является ссылкой на экземпляр структуры.
    ");
    let example = User {
        age: 30
    };
    println!("Возраст меньше нуля? {}", example.is_negative());
}

struct User {
    age: i32
}

impl User {
    fn is_negative(&self) -> bool {
        
        if self.age < 0 {
            return true;
        }
        else {
            return false;
        }

    }
}
