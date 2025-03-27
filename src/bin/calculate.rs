use std::io;

fn main() {
    println!("Первая попытка написать колькулятор самостоятельно");
    gate_calc();
}

fn gate_calc(){
    let mut x = String::new();
    let mut y = String::new();
    let mut z = String::new();

    println!("Введите первое число: ");
    io::stdin().read_line(&mut x).expect("Не получилось считать число");
    let x:f32 = x.trim().parse().expect("Не удалось преобразовать в число");
    println!("Введите второе число: ");
    io::stdin().read_line(&mut y).expect("Не получилось считать число");
    let y:f32 = y.trim().parse().expect("Не удалось преобразовать в число");
    println!("Введите действие (+, -, *, /): ");
    io::stdin().read_line(&mut z).expect("Не получилось считать знак действия");
    let z:&str = z.trim();

    let res = core(x, y, z);
    if res == None {
        println!("Что-то пошло не так");
    }
    else {
        println!("Результат: {}", res.unwrap());
    }
}

fn core(x:f32, y:f32, z:&str) -> Option<f32> {
    match z {
        "+" => Some(x + y),
        "-" => Some(x - y),
        "*" => Some(x * y),
        "/" => if y == 0.0 {
            panic!("Деление на ноль");
        } else { 
        Some(x / y)
        },
        _ => panic!("Неизвестная операция"),
    }
}
