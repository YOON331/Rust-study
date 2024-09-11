use std::io;

fn main() {
    println!(" ~~~ Fibonacci Program ~~~\nEnter a number.(-1 or lower: quit)");

    loop {
        let mut num = String::new();

        io::stdin().read_line(&mut num).expect("Failed to read line.");

        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input error.");
                continue
            }
        };

        if num < 0 {
            println!("The program will now finish.");
            break;
        } else {
            let result = get_fibo(num);
            println!("Result is {result}");
        }
    }
}

fn get_fibo(x:i32) -> i32 {
    if x == 0 {
        0
    } else if x == 1 {
        1
    } else {
        get_fibo(x-1) + get_fibo(x-2)
    }
}