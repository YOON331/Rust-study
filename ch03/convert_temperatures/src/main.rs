use std::io;

fn main() {
    loop {
        println!(" ~~~ Convert Temperatures Program ~~~\nChoose a menu option\n1. F to C\n2. C to F\n3. Quit\n");
        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let user_input: f64 = match user_input.trim().parse() {
            Ok(num) => {
                if num == 3.0 {
                    println!("The program will now finish.\n");
                    break;
                } else if num < 1.0 || num > 3.0 {
                    println!("Input error. please enter a menu option.\n");
                    continue;
                } else {
                    num
                }
            },
            Err(_) => {
                println!("Input error. please enter a menu option.\n");
                continue;
            }
        };
        
        println!("Please enter the temperature to convert >>> ");

        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");

        let number: f64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if user_input == 1.0 {
            let convert_num = f_to_c(number);
            println!("{:.2}°F is converted to {:.2}°C\n", number, convert_num);
        } else if user_input == 2.0 {
            let convert_num = c_to_f(number);
            println!("{:.2}°C is converted to {:.2}°F\n", number, convert_num);
        }
        
    }
}

fn f_to_c (x: f64) -> f64 {
    (x-32.0) * 5.0/9.0
}

fn c_to_f (x: f64) -> f64 {
    (x*9.0/5.0) + 32.0 
}