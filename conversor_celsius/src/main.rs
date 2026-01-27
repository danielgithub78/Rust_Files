/*Es una aplicacion que convierte una temperatura ingresada 
en Fahrenheit a Celsius */

use std::io;

fn convertir_a_celsius(fahrenheit: f32) -> f32 {
    return (fahrenheit - 32.0) * 5.0 / 9.0;
}

fn main() {
    println!("Conversor de temperatura de Fahrenheit a Celsius!\n");
    
    println!("\nIngrese la temperatura en ºF:");
    let mut fahrenheit: String = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Error al leer la temperatura");
    
    let fahrenheit:f32 = match fahrenheit
        .trim()
        .parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor ingrese una temperatura válida en grados Fahrenheit.");
                return;
            }
        };
   
    let celsius = convertir_a_celsius(fahrenheit);
    
    println!("Su equivalente es: {celsius:.2}ºC");
}