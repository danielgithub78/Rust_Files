mod motor;
mod personajes;

fn main() {
    crate::motor::iniciar_graficos();
    personajes::guerrero::atacar("Orco");
   //println!("Hello, world!");
}
