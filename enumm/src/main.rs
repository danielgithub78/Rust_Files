/* estoy probando los enum, con un ejemplo de componentes de una Pc*/

enum ComponentesPc {
    Teclado(String),
    Mouse(String),
    Parlantes(String),
    Monitor(String),
    Cpu,
    Ram(String),
}

fn Informar_Componente(componente: ComponentesPc) {
    match componente {
        ComponentesPc::Teclado(marca)=> println!("es un Teclado: {}", marca),
        ComponentesPc::Ram(marca)=> println!("es una memoria Ram: {}", marca),
        ComponentesPc::Monitor(marca)=>println!("es un Monitor {}",marca),
        _=> println!("es otro componente de la Pc"),
    }
}

fn main() {
    let mon_1 = ComponentesPc::Monitor;
    let cpu_1 = ComponentesPc::Cpu;
    let teclado_1 = ComponentesPc::Teclado;
    Informar_Componente(ComponentesPc::Monitor(String::from("Samsung SyncMaster 3, 15p")));
    Informar_Componente(ComponentesPc::Teclado(String::from("Logitech 105 Teclas")));
    Informar_Componente(ComponentesPc::Ram(String::from("Kingstong 3200mhz 16 GB")));
    Informar_Componente(cpu_1);

    let valor: Option<u32>= Some(15);
    match valor {
        Some(numero) => println!("El valor es: {}", numero),
        None=> println!("No hay numero"),
    }

    //println!("Hello, world!");

    let valor2: Option<u8> = Some(10u8);
        if let Some(num) = valor2 {
            println!("El numero es: {}", num);
        }   else {
                println!("No hay numero");
            }

}
