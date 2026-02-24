mod seguridad;
mod habitaciones;

fn main() {
    // 2. Encender la alarma
    crate::seguridad::activar_alarma();

    // 3. Preparar el desayuno pasando por las habitaciones y la cocina
    crate::habitaciones::cocina::preparar_desayuno();    
    
    
    //println!("Hello, world!");
}
