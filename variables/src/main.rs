fn main() {
    let mut x = 5;//probando mut y sin mut para ver como funciona
    let cc:char = 'd';
    let q: char ='x';
    let tupla: (&str, f32, u32, char) = ("Daniel", 12.56, 55, 'r'); //definicion de una tupla
    let m: (&str, f32, u32, char) = tupla;
    let (b,c,n,l)  = tupla; //desestructuracion de la tupla
    let zz: &str = tupla.0; //acceso a un elemento de la tupla
    let arreglo_stack:[u32;6] = [1,5,8,4,9,23]; //tipo stack, se conoce el tamaño y tipo antes de compilar. se poneen tipo u32 seguido del tamaño del arreglo, en este caso es de 6 elementos.
    let arreglo_heap: Vec<u32> = vec![2,5,9,3,2,1,67,87,21,43]; //no hace falta poner la dimension del arreglo, se crea en tiempo de ejecucion.
    

    println!("El valor del 2do elemenot de la tupla es: {}", c);
    println!("El valor de unatupla es: {}", m.1); //acceso a un elemento de la tupla
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of c is: {}", cc);
    println!("el valor de la 3ra posicion del arreglo es: {}", arreglo_heap[2]);



}