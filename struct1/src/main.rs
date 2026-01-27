/*prueba de struct, como funciona */

#[derive(Debug)]
struct Persona {
    nombre: String,
    apellido: String,
    edad: u8,
    telefono: String,
    domicilio: String,
    genero: char,
    estado_civil: char,
}

fn main() {
    let mut persona1: Persona = Persona {
        nombre: String::from("Pepe"),
        apellido: String::from("Garcia"),
        edad: 47,
        telefono: String::from("123456"),
        domicilio: String::from("calle falsa 123"),
        genero: 'M',
        estado_civil: 'S',
    };

    if persona1.nombre == "Juan" {
        println!("Hola {}", persona1.nombre);
    }

    persona1.nombre = String::from("Esteban");
    println!("La Persona es: {:#?}", persona1);
}
