
pub fn activar_alarma() {
    println!("Alarma activada");
    if !verificar_aberturas(){
        println!("Hay una puerta o ventana abierta");
        llamar_a_la_policia();
    }
}

pub fn desactivar_alarma() {
    println!("Alarma desactivada");
}

fn llamar_a_la_policia() {
    println!("Llamando a la policia...");
}

fn verificar_aberturas()->bool {
    println!("Verificando aberturas...");
    true //todas las aberturas estan cerradas, suponemos.
}