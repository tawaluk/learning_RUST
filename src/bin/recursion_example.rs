fn main() {
// Тестирую рекурсию
    let x = 50;
    let y = 500;
    let new_x_y = test_req(x,y);
    println!("x is {} and y is {}", new_x_y.0, new_x_y.1);
}

fn test_req(mut x: i32, y: i32) -> (i32, i32) {

    if x < y {
        x += 40;
        println!("x is {}", x);
        return test_req(x, y);
    };
    (x, y)
}
