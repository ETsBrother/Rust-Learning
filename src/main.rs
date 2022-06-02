fn main() {
    println!("Hello world");

    let mut x: i32 = 12;
    let mut y: f32 = 2.0;
    x += 5;
    y *= 1.5;
    println!("x/y = {}", x / y as i32);

    if x == 4 {
        println!("x is 4");
    } else if x <= 3 {
        println!("x is less than 4");
    } else {
        println!("x is greater than 4")
    }

    let mut a: [i32; 4] = [1, 2, 3, 4];
    a[2] = 4;
    println!("len of a is {}", a.len());
    for elem in a {
        print!("{} ", elem);
    }
    println!();
    
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            print!("FizzBuzz ");
        } else if i % 3 == 0 {
            print!("Fizz ");
        } else if i % 5 == 0 {
            print!("Buzz ");
        } else {
            print!("{} ", i);
        }
    }
    println!();
}
