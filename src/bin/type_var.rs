fn main() {
    println!("Hello");

    let info = String::from(
        r#"
        Например, это многострочный комментарий.
        Все строки, включая эту, будут включены в строку.
        Тут нет необходимости использовать экранирование.
        Комбинация # на краях строк используется для подавления экранирования,
        а также для предотвращения разбивки строк.
        Правильное название этой строки: Raw string.
        Это удобно использовать для многострочных строковых литералов и регулярных выражений.
        Сам синтаксис let info = String::from("..."); используется для создания строки из текста.
        fn - это функция, которая создает строку из текста.
        fn main - это функция, которая является точкой входа в программу. Она вызывается при запуске программы. Она обязательна.
        fn main() - это функция, которая принимает аргументы командной строки. Тут она не принимает ничего.
        {} - это символы, которые указывают тело функции. В них может быть любой код.
        ; - это символ, который указывает конец выражения. В Rust почти каждое выражение должно заканчиваться символом ;
        let - это ключевое слово для объявления неизменяемой переменной. ( let mut - для создания изменяемой переменной)
        info - имя переменной
        String - тип переменной, который указывает, что это строка, в памяти лежит в куче. Это важно!
        ::from - это вызов метода from у типа String. Фактически - это вызов функции String::from.
        r - это маркер, который говорит, что это сырой строковый литерал. Это значит, что все символы в строке будут включены в строку, включая символы новой строки.
        # - это символ, который говорит, что это многострочный комментарий. Это значит, что все строки, включая эту, будут включены в комментарий.
        println - это функция, которая выводит текст на консоль. Она принимает аргументы и выводит их на консоль,
        но не возвращает ничего. Она также может принимать форматированные строки. Если бы аргументов было 2,
        то функция бы выглядела так: println!("{} {}", arg1, arg2);

        "#
    );
    println!("{}", info);

    let stack_var = String::from(
        r#"
        Все типы дынных в Rust можно поделить на 2 категории: стековые и кучевые.
        Стековые типы данных хранятся в стеке и имеют фиксированный размер.
        Стековые типы данных:
        - i32 - целое число 32 бита ( может быть i8, i16, i64, i128, isize)
          Пример обьявления - let x: i32 = 5;
        - u32 - число без знака 32 бита ( может быть u8, u16, u64, u128, usize)
          Пример обьявления - let x: u32 = 5;
        - f64 - число с плавающей запятой ( может быть f32, f64)
          Пример обьявления - let x: f64 = 5.0;
        - bool - логический тип данных ( может быть true или false)
          Пример обьявления - let x: bool = true;
        - char - символ ( может быть любой символ в формате Unicode)
          Пример обьявления - let x: char = 'a';
        - tuple - кортеж ( в котором могут быть любые типы данных. Фактически кортеж формирует свой неизменяемый тип данных!)
          Пример обьявления - let x: (i32, f64, u8) = (500, 6.4, 1);
        - array - массив ( в котором могут быть элементы только одного типа данных. Имеет фиксированный размер.)
          Пример обьявления - let x: [i32; 5] = [1, 2, 3, 4, 5];
        Кучевые типы являются более сложными для понимания из за необходимости понимания таких принципов как владение
        Кучевые типы данных:
        - Srting - строка ( динамически изменяемая, гарантирует, что строка всегда будет валидной UTF-8 )
          Пример обьявления - let x: String = String::from("Hello, world!");
        - Vec<T> - вектор ( динамически изменяемый массив. Может быть изменен по размеру. Имеет фиксированный тип данных. )
          <T> - это обобщенный тип данных. Это значит, что вектор может содержать элементы любого типа данных.
        - Box<T> - коробка ( указатель на кучу. Может быть изменен по размеру. Имеет фиксированный тип данных. )
        - Mutex<T> - мьютекс ( рид-лок - это тип данных, который позволяет одному потоку изменять данные, а другим потокам только читать данные. )
        - RwLock<T> - рид-лок ( рид-лок - это тип данных, который позволяет одному потоку изменять данные, а другим потокам только читать данные. )
        
        "#
    );
}
