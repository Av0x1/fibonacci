use std::io;

fn main() {
    let mut input_number: i32;

    println!("Die wie vielte Fibonacci-Zahl wollen Sie ermitteln? Bitte geben Sie eine Zahl ein: ");
    loop {
        let user_input = get_user_input();
        input_number = convert_number(user_input);

        if input_number == -1 {
            println!("Bitte eine gültige Zahl eingeben.");
            continue;
        }
        else {
            break;
        }
    }

    println!("Die Zahl ist: {input_number}.");
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