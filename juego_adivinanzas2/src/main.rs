/* la palicacion genera un numero random entre 1 y 100, luego pide que numero es
el que genero la app, devolviendo bajo alto o igual respecto al numero que creemos 
que es. si es acertado imprime mensaje y sale de la app */

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivina el numero!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("El numero secreto es: {}", secret_number);

    loop {
        println!("Ingrese usted el numero que cree que es: ");
    
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)//lee asigna a la posicion de memoria de guess lo escrito por teclado
            .expect("Fallo en leer la linea");
    
        let guess: u32 = match guess.trim().parse() { //asigna a guess el numero escrito con string
            Ok(num) => num,
            Err(_) => {
                println!("Por favor ingrese un numero valido.");
                continue;
            }
        };

        //println!("El numero que dije yo es: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Demasiado bajo!"),
            Ordering::Greater => println!("Demasiado alto!"),
            Ordering::Equal => {
                println!("¡Lo adivinaste el numero, era: {}!", guess);
                break;
            }
        }
    }
}