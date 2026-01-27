
use std::io;

fn main() {
    /* se ingresara texto por teclado y se analizara la longitud
    de la 1ra palabra. si no hay espacio vacio todo lo que se ingreso
    se supone que conforma una palabra y se imprime su longitud */

    let mut texto = String::new();
    println!("Ingrese un texto: \n");
 
    io::stdin()
    .read_line(&mut texto)
    .expect("Fallo en leer la linea");

    let longi= longitud(&texto);
    println!("La longitud de la 1ra palabra es: {}", longi);
    let palabra = &texto[0..longi]; //solucion de encontrar la primera palabra usando slice!    
    println!("La 1ra palabra es: {palabra}");
}

fn longitud(texto:&String)->usize {
    //voy a pasarlo a bytes, asignando cada letra a un espacio de un vector.
    let bytes = texto.as_bytes();
    for (i, &elemento) in bytes.iter().enumerate() {
    //buscare el espacio vacio de la palabra
        if elemento == b' ' {
           return i; 
        }
    }
    //sino encuentra espacio en medio todo es string es una palabra.
    // y devuelve el tamaño total.
    texto.len()
}