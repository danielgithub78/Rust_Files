/*prueba de struct, como funciona */

fn main() {
    struct Persona {
        nombre: String,
        apellido: String,
        edad: u8,
        telefono: String,
        domicilio: String,
        genero: char,
        estado_civil: char,
    }
}

let mut persona1 =Persona;
persona1.nombre="Pepe";
persona1.apellido="Garcia";
persona1.edad=47;
persona1.telefono="123456";

if persona1.nombre=="Juan" {
    println!("Hola {}", persona1.nombre);
}

persona1.nombre="Esteban";
