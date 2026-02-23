mod inventario;
mod compras;

fn main() {
    let producto="Teclado Mecanico HP";
    let precio=1500;

    if crate::inventario::consultar_stock(producto){
        // pagos::procesar_tarjeta(precio); si dejo este le estariamos cobrando 2 veces al cliente
        compras::realizar_compra(producto, precio);
    }
    //println!("Hello, world!");
}