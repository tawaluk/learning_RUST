fn main() {
    println!("Hello, world!");
    integers_i();
    integers_f();
    integers_u();
    integers_c();
    math_functions();
    number_operations();
    type_conversions();
}

// Типы данных:

// Числа
fn integers_i() {
    println!("\n--- Целочисленные со знаком (i8-i128) ---");
    let a: i16 = 2;
    let b: i16 = 3; // default

    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {}", a, b, a / b);      // целочисленное деление
    println!("{} % {} = {}", a, b, a % b);      // остаток от деления
    
    // Побитовые операции
    println!("{} & {} = {}", a, b, a & b);      // AND
    println!("{} | {} = {}", a, b, a | b);      // OR  
    println!("{} ^ {} = {}", a, b, a ^ b);      // XOR
    println!("!{} = {}", a, !a);                // NOT
    println!("{} << 1 = {}", a, a << 1);        // сдвиг влево
    println!("{} >> 1 = {}", a, a >> 1);        // сдвиг вправо
    
    // Переполнение
    let max_i8: i8 = 127;
    // let overflow = max_i8 + 1; // panic в debug режиме
    let wrapped = max_i8.wrapping_add(1);
    println!("127_i8 + 1 = {} (wrapping)", wrapped);
}

fn integers_f() {
    println!("\n--- Числа с плавающей точкой (f32, f64) ---");
    
    let x: f64 = 3.5;
    let y: f64 = 2.0;
    println!("{} + {} = {}", x, y, x + y);
    println!("{} - {} = {}", x, y, x - y);
    println!("{} * {} = {}", x, y, x * y);
    println!("{} / {} = {}", x, y, x / y);
    println!("{} % {} = {}", x, y, x % y);
    
    // Специальные значения
    println!("NaN: {}", f64::NAN);
    println!("Бесконечность: {}", f64::INFINITY);
    println!("-Бесконечность: {}", f64::NEG_INFINITY);
    
    // Проверки
    println!("{} is NaN: {}", f64::NAN, f64::NAN.is_nan());
    println!("{} is finite: {}", x, x.is_finite());
}

fn  integers_u() {
    println!("\n--- Беззнаковые целые (u8-u128) ---");
    
    let a: u32 = 10;
    let b: u32 = 3;
    
    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {}", a, b, a / b);
    println!("{} % {} = {}", a, b, a % b);
    
    // Методы проверки переполнения
    match a.checked_add(b) {
        Some(result) => println!("{} + {} = {} (checked)", a, b, result),
        None => println!("Переполнение!"),
    }
    
    // Возведение в степень
    println!("2^8 = {}", 2u32.pow(8));
    println!("4^0.5 = {}", (4.0f64).sqrt());
}

// реализация комплексных чисел
#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
    
    fn add(self, other: Self) -> Self {
        todo!()
    }
    
    fn multiply(self, other: Self) -> Self {
        todo!()
    }
    
    fn modulus(self) -> f64 {
        todo!()
    }
}

fn integers_c() {
    let ex_1 = Complex::new(3.0, 4.0);
    let ex_2 = Complex::new(1.0, 2.0);
    println!("{}:{}", ex_1.real, ex_2.imag);
}

fn math_functions() {
    println!("\n--- Математические функции ---");
    
    let x: f64 = 2.0;
    
    println!("sqrt({}) = {}", x, x.sqrt());
    println!("{0}^3 = {1}", x, x.powi(3));
    println!("e^{} = {:.3}", x, x.exp());
    println!("ln({}) = {:.3}", x, x.ln());
    println!("log10({}) = {:.3}", x, x.log10());
    
    // Тригонометрия
    let angle = 45.0_f64.to_radians();
    println!("sin(45°) = {:.3}", angle.sin());
    println!("cos(45°) = {:.3}", angle.cos());
    println!("tan(45°) = {:.3}", angle.tan());
    
    // Константы
    println!("π = {:.5}", std::f64::consts::PI);
    println!("e = {:.5}", std::f64::consts::E);
}

fn number_operations() {
    println!("\n--- Разные операции с числами ---");
    
    // Сравнения
    let (x, y) = (10, 20);
    println!("{} == {}: {}", x, y, x == y);
    println!("{} != {}: {}", x, y, x != y);
    println!("{} < {}: {}", x, y, x < y);
    println!("{} >= {}: {}", x, y, x >= y);
    
    // Приведение типов в выражениях
    let int_val: i32 = 42;
    let float_val: f64 = int_val as f64;
    println!("{} as f64 = {}", int_val, float_val);
    
    // Округление
    let pi: f32 = 3.14159;
    println!("floor({}) = {}", pi, pi.floor());
    println!("ceil({}) = {}", pi, pi.ceil());
    println!("round({}) = {}", pi, pi.round());
    println!("trunc({}) = {}", pi, pi.trunc());
}

// Преобразования типов
fn type_conversions() {
    println!("\n--- Преобразования типов ---");
    
    // Безопасные преобразования
    let small: i16 = 1000;
    let large: i32 = small as i32;
    println!("{} as i32 = {}", small, large);
    
    // Потеря точности
    let float: f64 = 3.99;
    let int: i32 = float as i32;
    println!("{} as i32 = {} (потеря точности)", float, int);
    
    // Опасные преобразования
    let big: i32 = 300;
    let small: i8 = big as i8;
    println!("{} as i8 = {} (переполнение)", big, small);
}
