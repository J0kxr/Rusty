use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("!Zahlenraten!");

    let number_to_guess = rand::thread_rng().gen_range(1..=100);

//    println!("Die Geheimzahl ist: {number_to_guess}");

    loop {
        println!("Gib eine Zahl ein: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fehler beim lesen der Zeile!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Deine Schätzung ist die Zahl: {}", guess);

        match guess.cmp(&number_to_guess) {
            Ordering::Less => println!("Zu klein!"),
            Ordering::Greater => println!("Zu groß!"),
            Ordering::Equal => {
                println!("Du hast gewonnen!");
                break;
            }
        }
    }
}
