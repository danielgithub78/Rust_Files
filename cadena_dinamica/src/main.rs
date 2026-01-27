fn main() {
    let mut cadena = String::from("Hola"); // Crea una cadena dinamica que se maneja en el heap de memoria    
    cadena.push_str(",soy Daniel");
    println!("{}",cadena);
}
