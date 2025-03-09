fn main() {
    println!("Ветвления");

    // Простейший пример ветвления
    let x = 5;

    if x == 5 {
        println!("x = 5");
    }
    else if x == 1 {
        println!("x = 1");
    }
    else {
        println!("x не 1 и не 5");
    };

    // Пример ветвления с присвоением переменной

    let x = 5;
    let y = if x == 5 {
        10
    }
    else {
        15
    };
    println!("Y: {}", y);

    // Пример ветвления с вложенным ветвлением

    let xz = 5;
    let yz = if xz == 5 {
        if xz == 1 {
            10
        } else {
            23
        }
    } else {
        0
    };
    println!("Yz: {}", yz);

}
