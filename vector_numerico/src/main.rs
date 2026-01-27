/*enemos un  arreglo de 10 elementos, y debemos ingresar por teclado un valor
entero entre 0 y 9, y mostrar el valor que se encuentra en esa posicion del
 arreglo*/

use std::io;

fn main() {
    
    let v_num: [u32;10] = [2,4,7,9,2,4,8,0,21,78];
       
    println!("\n Aplicacion que devuelve el valor de un elemento de un arreglo segun la posicion ingresada por el usuario (entre 0 y 9) \n");

    println!("Ingrese un numero entre 0 y 9: ");
    let mut indice = String::new();
    io::stdin()
        .read_line(&mut indice)
        .expect("Fallo en leer el dato");

    let indice: usize = indice
        .trim()
        .parse()
        .expect("Por favor ingrese un numero valido entre 0 y 9");
    
    println!("\n El contenido del arreglo en la posicion {} es: {}", indice, v_num[indice]);
    
        }
