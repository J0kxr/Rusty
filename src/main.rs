use std::io;


fn main() {
    println!("!Zahlenraten!");

    
    println!("Gib eine Zahl ein: ");


    let mut guess = String::new();


    io::stdin()
        .read_line(&mut guess)
        .expect("Fehler beim lesen der Zeile!");

    println!("Deine Schätzung ist die Zahl: {}", guess);

}
