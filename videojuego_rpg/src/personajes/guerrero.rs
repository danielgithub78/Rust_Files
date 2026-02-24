pub fn atacar(enemigo: &str) {
    afilar_espada(); // Puede llamarla porque están en el mismo archivo
    println!("¡El guerrero ataca al {} con 50 de daño!", enemigo);
}

// Ojo: Esta función NO tiene 'pub'
fn afilar_espada() {
    println!("*Sonido de afilar metal*");
}