/* probando struct y metodos del struct, en este caso struct cuadrado, y sus
metodos area, perimetro y volumen */

#[derive(Debug)]

struct Cuadrado {
    lado: u32,
}

impl Cuadrado {
    fn area(&self) -> u32 {
        self.lado * self.lado
    }

    fn perimetro(&self) -> u32 {
        self.lado * 4
    }

    fn volunen(&self) -> u32 {
        self.lado * self.lado * self.lado
    }

    fn get_lado(&self) -> u32 {
        self.lado
    }

    fn set_lado(&mut self, lado: u32) {
        self.lado = lado;
    }
}

fn main() {
    let mut cuadrado = Cuadrado { lado: 10 };
    println!("\nEl area del cuadrado es: {}", cuadrado.area());
    println!("El perimetro del cuadrado es: {}", cuadrado.perimetro());
    println!("El volunen del cuadrado es: {}", cuadrado.volunen());

    println!("\n {cuadrado:?}");
    println!("El lado del cuadrado es: {}", cuadrado.get_lado());
    cuadrado.set_lado(21);
    println!("\n {cuadrado:?}");
    println!("El lado del cuadrado es: {}", cuadrado.get_lado());
}

