pub mod pagos;

pub fn realizar_compra(item:&str, precio:u32) {
    println!("Iniciando compra de: {}", item);
    pagos::procesar_tarjeta(precio);
}