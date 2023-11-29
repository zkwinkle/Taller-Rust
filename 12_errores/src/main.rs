// Resolver las tareas dentro de los siguientes módulos

mod ejercicio1;
mod ejercicio2;

// El ejercicio 3 es arreglar este main

// Este es un programa que intenta usar una versión completada de la
// función `total_cost` del ejercicio anterior. ¡Sin embargo, no está funcionando!
// ¿Por qué no? ¿Qué debemos hacer para arreglarlo?
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
}
