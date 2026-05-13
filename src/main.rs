//mod println;
use std::io::{self, Write};
use std::process::Command;


/*fn main() {
    println::display_message();
    println!("Hello, world!");
}
*/

/* Gestion des erreurs propre, unwrap() est à éviter car il panique en cas d'erreur,
ici on gère les erreurs de manière plus élégante avec match

match io::stdout().flush() {
    Ok(_) => { /* success */ }
    Err(e) => {
        eprintln!("Erreur d'écriture : {}", e);
        continue;
    }
 */

fn main() {
    loop {
        print!("$ ");
        match io::stdout().flush() {
            Ok(_) => { /* success */ }
            Err(e) => {
                eprintln!("Erreur d'écriture : {}", e);
                continue;
            }
        };

        let mut input = String::new();
        match io::stdin().read_line( &mut input) {
            Ok(_) => { /* success */}
            Err(e) => {
                eprintln!("Erreur de lecture : {}", e);
                continue;
            }
        }
        let _input = input.trim();

        let args: Vec<&str> = _input.split_whitespace().collect();
        match args.is_empty() {
            true => continue,
            false => {}
        }

        let command = args[0];
        let arguments = &args[1..];

        Command::new(command)
            .args(arguments)
            .spawn()
            .unwrap_or_else(|e| {
                eprintln!("Erreur d'exécution de la commande '{}': {}", command, e);
                std::process::exit(1);
            });
    }
}