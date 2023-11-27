// INSTRUCCIONES:
// 1. Edite las partes marcadas para lograr que el código compile

fn main() {
    // Código ejemplo correcto, no editar
    let a = 5;
    println!("a tiene el valor {}", a);

    // Editar esta sección
    // *************************
    let b;
    if b == 10 {
        println!("b es diez! :D");
    } else {
        println!("b no es diez! :(");
    }

    let c: i32;
    println!("Número c '{}'", c);

    let d = 3;
    println!("Número d {}", d);
    d = 5; // NO cambiar esta línea
    println!("Número d {}", d);

    let número = "T-R-E-S"; // NO cambiar esta línea
    println!("Deletrear número : {}", número);
    número = 3; // NO renombre esta variable (pero puede modificar la línea)
    println!("número más dos es: {}", número + 2);

    const NÚMERO_CONSTANTE = 69;
    println!("Número constante {}", NÚMERO_CONSTANTE);
    // *************************
}
