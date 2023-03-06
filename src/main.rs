use std::io;

fn main() {
    let mut input_number: i32;

    println!("Die wievielte Fibonacci-Zahl wollen Sie ermitteln? Bitte geben Sie eine Zahl ein: ");
    loop {
        let user_input = get_user_input();
        input_number = convert_number(user_input);

        if input_number < 1 {
            println!("Bitte eine gültige ganzzahlige Zahl über 0 eingeben.");
            continue;
        } else {
            break;
        }
    }

    let fibonacci = calculate_fibonacci(input_number);

    println!("Die Zahl ist: {fibonacci}.");
}

fn get_user_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Fehler beim Laden des Inputs.");

    return input;
}

fn convert_number(input: String) -> i32 {
    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };

    return input;
}

fn calculate_fibonacci(number: i32) -> i32 {
    if number == 1 {
        return number;
    }

    let mut counter = 0;
    let mut num1 = 0;
    let mut num2 = 1;
    let mut fibonacci = 0;

    loop {
        if counter == number - 1 {
            break;
        }

        fibonacci = num1 + num2;
        num1 = num2;
        num2 = fibonacci;

        counter += 1;
    }

    return fibonacci;
}
