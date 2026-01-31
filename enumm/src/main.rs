/* estoy probando los enum, con un ejemplo de componentes de una Pc*/

enum ComponentesPc {
    Teclado,
    Mouse,
    Parlantes,
    Monitor,
    Cpu,
    Ram,
}

fn Un_Componente(componente:ComponentesPc){
    match componente {
        ComponentesPc::Teclado=>println!("es un Teclado"),
        _=>println!("es otro componente"),
    }
}

fn main() {
    let mon_1=ComponentesPc::Monitor;
    let cpu_1=ComponentesPc::Cpu;
    let teclado_1=ComponentesPc::Teclado;
    Un_Componente(mon_1);
    Un_Componente(teclado_1);
    Un_Componente(cpu_1);

    //println!("Hello, world!");
}