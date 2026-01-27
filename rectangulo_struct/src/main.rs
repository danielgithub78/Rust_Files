#[derive(Debug)]
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

fn area(rectangulo: &Rectangulo) -> u32 {
    rectangulo.ancho * rectangulo.alto
}

fn main() {
    let rect_1 = Rectangulo {
        ancho: 10,
        alto: 25,
    };

    println!("El area del rectangulo es {}", area(&rect_1));

    //println!("Los datos de rec_1 son: {rect_1:?}"); //muestra la estructura en una linea.

    //println!("Los datos de rec_1 son: {rect_1:#?}"); //muestra la estructura en un campo debajo del otro.

    dbg!(&rect_1);
}
