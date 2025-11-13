fn main() {
    // 1. СОЗДАНИЕ ВЕКТОРОВ
    let mut vec1: Vec<i32> = Vec::new();          // пустой вектор
    let mut vec2 = vec![1, 2, 3, 4, 5];          // макрос vec!
    
    println!("1. Создание:");
    println!("vec1: {:?}", vec1);
    println!("vec2: {:?}", vec2);
    
    // 2. ДОБАВЛЕНИЕ ЭЛЕМЕНТОВ
    vec1.push(10);                               // добавить в конец одно значение
    vec1.extend([1, 2, 3]);                 // добавить в конец массив значений
    vec1.extend(23..=10);                   // добавить в конец массив значений из итератора
    vec1.insert(1, 9999);                          // вставить на позицию 1
    
    println!("\n2. После добавления:");
    println!("vec1: {:?}", vec1);
    
    // 3. ДОСТУП К ЭЛЕМЕНТАМ
    println!("\n3. Доступ к элементам:");
    println!("Первый: {:?}", vec1.first());      // Option<&T>
    println!("Последний: {:?}", vec1.last());
    println!("По индексу 1: {}", vec1[1]);       // паника при ошибке
    println!("Безопасный доступ: {:?}", vec1.get(1)); // Option<&T>
    
    // 4. ИЗМЕНЕНИЕ ЭЛЕМЕНТОВ
    if let Some(elem) = vec1.get_mut(0) {
        *elem = 100;
    }
    vec1[1] = 150;
    
    println!("\n4. После изменений:");
    println!("vec1: {:?}", vec1);
    
    // 5. РАЗМЕР И ЕМКОСТЬ
    println!("\n5. Размер и емкость:");
    println!("Длина: {}", vec1.len());
    println!("Емкость: {}", vec1.capacity());
    println!("Пустой: {}", vec1.is_empty());
    
    // 6. УДАЛЕНИЕ ЭЛЕМЕНТОВ
    vec1.pop();                                  // удалить последний
    vec1.remove(0);                              // удалить по индексу
    
    println!("\n6. После удаления:");
    println!("vec1: {:?}", vec1);
    
    // 7. ПОИСК ЭЛЕМЕНТОВ
    println!("\n7. Поиск:");
    let index = vec2.iter().position(|&x| x == 3);
    println!("Позиция элемента 3: {:?}", index);
    
    let contains = vec2.contains(&4);
    println!("Содержит 4: {}", contains);
    
    // 8. СОРТИРОВКА
    let mut numbers = vec![5, 2, 8, 1, 9];
    numbers.sort();
    
    println!("\n8. После сортировки:");
    println!("numbers: {:?}", numbers);
    
    // 9. ОБХОД ВЕКТОРА
    println!("\n9. Обход вектора:");
    
    println!("Итерация по ссылкам:");
    for num in &vec2 {
        print!("{} ", num);
    }
    println!();
    
    println!("Итерация с изменением:");
    for num in &mut vec1 {
        *num += 10;
        print!("{} ", num);
    }
    println!();
    
    println!("Итерация с индексами:");
    for (i, num) in vec2.iter().enumerate() {
        println!("vec2[{}] = {}", i, num);
    }
    
    // 10. ФИЛЬТРАЦИЯ И ПРЕОБРАЗОВАНИЕ
    println!("\n10. Фильтрация и преобразование:");
    
    let even_numbers: Vec<i32> = vec2.iter()
        .filter(|&&x| x % 2 == 0)
        .cloned()
        .collect();
    println!("Четные числа: {:?}", even_numbers);
    
    let squared: Vec<i32> = vec2.iter()
        .map(|x| x * x)
        .collect();
    println!("Квадраты: {:?}", squared);
    
    // 11. ОБЪЕДИНЕНИЕ ВЕКТОРОВ
    println!("\n11. Объединение векторов:");
    
    let mut combined = vec1.clone();
    combined.extend(vec2.iter());
    println!("Объединенный: {:?}", combined);
    
    // 12. СРЕЗЫ (slices)
    println!("\n12. Работа со срезами:");
    
    let slice = &vec2[1..4]; // элементы с 1 по 3
    println!("Срез [1..4]: {:?}", slice);
    
    // 13. ДВУМЕРНЫЙ ВЕКТОР
    println!("\n13. Двумерный вектор:");
    
    let matrix: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    
    for row in &matrix {
        for num in row {
            print!("{} ", num);
        }
        println!();
    }
    
    // 14. ПОЛЕЗНЫЕ МЕТОДЫ
    println!("\n14. Полезные методы:");
    
    let sum: i32 = vec2.iter().sum();
    println!("Сумма vec2: {}", sum);
    
    let max = vec2.iter().max();
    println!("Максимум: {:?}", max);
    
    vec1.reverse();
    println!("После reverse: {:?}", vec1);
    
    // 15. ОЧИСТКА И ИЗМЕНЕНИЕ РАЗМЕРА
    println!("\n15. Очистка и изменение размера:");
    
    let mut resize_vec = vec![1, 2, 3];
    resize_vec.resize(5, 100); // увеличить до 5, новые = 100
    println!("После resize: {:?}", resize_vec);
    
    vec1.clear();
    println!("После clear: {:?}", vec1);
    
    // 16. ВЕКТОР С РАЗНЫМИ ТИПАМИ ЧЕРЕЗ ENUM
    println!("\n16. Вектор с разными типами:");
    
    #[derive(Debug)]
    enum Value {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let mixed_vec = vec![
        Value::Int(42),
        Value::Float(3.14),
        Value::Text("hello".to_string()),
    ];
    
    for value in mixed_vec {
        println!("{:?}", value);
    }
}
