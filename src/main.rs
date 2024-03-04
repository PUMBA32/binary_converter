use std::io;

fn get_choice() -> i8 {
    let mut str_choice = String::new();
    let _ = io::stdin().read_line(&mut str_choice);
    let choice: i8 = str_choice.trim().parse().unwrap();
    match choice {
        1 => 1,
        2 => 2,
        _ => 3
    }
}

fn to_decimal(str_num: String) -> i32 {
    let mut res = 0;
    for (index, el) in str_num.chars().rev().enumerate() {
        if el == '1' {
            res += 2u32.pow(index as u32);
        }
    }
    res 
}

fn to_binary(str_num: String) -> String {
    let mut num: i32 = str_num.trim().parse().unwrap();
    let mut res = String::new();

    while num > 0 {
        if num % 2 == 0 {
            res.push_str("0");
        } else {
            num -= 1;
            res.push_str("1");
        }
        num /= 2;
    }
    
    res.chars().rev().collect::<String>()  // реверс строки
}

fn main() {
    loop {
        println!("Выбери действие: ");
        println!("\n[1] - x2 -> x10\n[2] - x10 -> x2\n[3] -> Выход");
        
        let choice = get_choice();

        if choice > 2 || choice < 1 {
            break;
        }

        let mut str_num = String::new();

        println!("Введите число для преобразования: ");
        match io::stdin().read_line(&mut str_num) {
            Ok(_) => {},
            Err(e) => println!("{}", e)
        }

        if choice == 1 {
            let result = to_decimal(str_num);
            println!("Результат: {}\n", result);
        } else {
            let result = to_binary(str_num);
            println!("Результат: {}\n", result);
        } 
    }
}