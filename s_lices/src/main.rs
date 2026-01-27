fn main() {

    let cadena = String::from("Hola soy daniel y estoy estudiando Rust");

    let parte1 = &cadena[0..4];
    println!("{}", parte1);

    let parte2 = &cadena[9..15];   
    println!("{parte2}");

    let convertir = cadena.as_bytes();
    println!("{}", convertir[2]);
}