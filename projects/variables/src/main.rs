fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {} y is: {} z is: {}", x, y, z);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("The value of first is: {}", first);
    // index over
    //let index_over = 10;
    //println!("The value of index over value is: {}", a[index_over]);
    another_function();
    another_function2(1, 2);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    if_test();

    let_if();
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32, y:i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn if_test() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, 2");
    }
}

fn let_if() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}
