/* estoy probando los enum, con un ejemplo de componentes de una Pc*/

enum ComponentesPc {
    Teclado(String),
    Mouse(String),
    Parlantes(String),
    Monitor,
    Cpu,
    Ram(String),
}

fn Informar_Componente(componente: ComponentesPc) {
    match componente {
        ComponentesPc::Teclado(marca)=> println!("es un Teclado: {}", marca),
        ComponentesPc::Ram(marca)=> println!("es una memoria Ram: {}", marca),
        _=> println!("es otro componente de la Pc"),
    }
}

fn main() {
    let mon_1 = ComponentesPc::Monitor;
    let cpu_1 = ComponentesPc::Cpu;
    let teclado_1 = ComponentesPc::Teclado;
    Informar_Componente(mon_1);
    Informar_Componente(ComponentesPc::Teclado(String::from("Logitech 105 Teclas")));
    Informar_Componente(ComponentesPc::Ram(String::from("Kingstong 3200mhz 16 GB")));
    Informar_Componente(cpu_1);

    //println!("Hello, world!");
}
