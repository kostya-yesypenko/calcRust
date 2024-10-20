use std::io::{self, Write};
use std::str::FromStr;

fn main() {
    loop {
        println!("Оберіть режим:");
        println!("1: Простий калькулятор");
        println!("2: Польська нотація");
        println!("exit: Вихід");

        let mut choice = String::new();
        io::stdout().flush().unwrap(); // Очищаємо буфер виводу
        io::stdin().read_line(&mut choice).expect("Не вдалося прочитати рядок");

        let choice = choice.trim();

        if choice.eq_ignore_ascii_case("exit") {
            break;
        }

        match choice {
            "1" => simple_calculator(),
            "2" => polish_notation_calculator(),
            _ => println!("Помилка: невірний вибір!"),
        }
    }
}

fn simple_calculator() {
    let mut memory: f64 = 0.0;

    loop {
        println!("Операція (+, -, *, /) або 'exit' для виходу:");
        let mut operation = String::new();
        io::stdout().flush().unwrap(); // Очищаємо буфер виводу
        io::stdin().read_line(&mut operation).expect("Не вдалося прочитати рядок");

        let operation = operation.trim();

        if operation.eq_ignore_ascii_case("exit") {
            break;
        }

        println!("Число:");
        let mut number_input = String::new();
        io::stdout().flush().unwrap(); // Очищаємо буфер виводу
        io::stdin().read_line(&mut number_input).expect("Не вдалося прочитати рядок");

        let number: f64 = match number_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Помилка: введіть дійсне число!");
                continue;
            }
        };

        match operation {
            "+" => memory += number,
            "-" => memory -= number,
            "*" => memory *= number,
            "/" => {
                if number != 0.0 {
                    memory /= number;
                } else {
                    println!("Помилка: ділення на нуль!");
                    continue;
                }
            }
            _ => {
                println!("Помилка: невідома операція!");
                continue;
            }
        }

        println!("Проміжний результат: {}", memory);
    }

    println!("Завершення простого калькулятора. Останній результат: {}", memory);
}

fn polish_notation_calculator() {
    println!("Введіть алгебраїчний вираз у польській нотації:");
    let mut expression = String::new();
    io::stdout().flush().unwrap(); // Очищаємо буфер виводу
    io::stdin().read_line(&mut expression).expect("Не вдалося прочитати рядок");

    let tokens: Vec<&str> = expression.trim().split_whitespace().collect();
    let result = evaluate_polish_notation(&tokens);

    match result {
        Some(value) => println!("Результат: {}", value),
        None => println!("Помилка при обчисленні виразу!"),
    }
}

fn evaluate_polish_notation(tokens: &[&str]) -> Option<f64> {
    let mut stack: Vec<f64> = Vec::new();

    // Проходимо токени в зворотному порядку
    for token in tokens.iter().rev() {
        if let Ok(number) = f64::from_str(token) {
            // Якщо токен - число, додаємо його в стек
            stack.push(number);
        } else {
            // Якщо токен - оператор
            if stack.len() < 2 {
                return None; // Недостатньо операндів
            }
            let left = stack.pop().unwrap();
            let right = stack.pop().unwrap();

            let result = match *token {
                "+" => left + right,
                "-" => left - right,
                "*" => left * right,
                "/" => {
                    if right != 0.0 {
                        left / right
                    } else {
                        return None; // Ділення на нуль
                    }
                }
                _ => return None, // Невідомий оператор
            };
            // Додаємо результат назад в стек
            stack.push(result);
        }
    }

    // Повертаємо остаточний результат
    stack.pop()
}
