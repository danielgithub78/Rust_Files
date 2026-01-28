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

    if persona1.nombre == "Pepe" {
        println!("Hola {}", persona1.nombre);
    }

    persona1.nombre = String::from("Esteban");
    println!("La Persona es: {:#?}", persona1);

    let user1=Persona{
        nombre: persona1.nombre.clone(),
        apellido: persona1.apellido.clone(),
        edad: persona1.edad,
        telefono: String::from("5678432"),
        domicilio: String::from("calle falsa 456"),
        genero: persona1.genero,
        estado_civil: persona1.estado_civil,
    };

    let mut user2=Persona{
        domicilio: String::from("calle falsa 321"),
        ..persona1
    };
    println!("La Persona es: {:#?}", user2);
}
