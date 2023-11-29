fn llámame(num: i32) {
    for i in 0..num {
        println!("Llamame #{}", i);
    }
}

fn arreglame(num:) {
    for i in 0..num {
        println!("Arreglame #{}", i + 1);
    }
}

fn suma(a: i32, b) -> {
    a + b
}

fn main() {

    llamame(4);
    
    arreglame(6);

    let x = 2;
    let y = 3;
    let z = suma(x,y);
    println!(
        "Multiplicación de {}*{}*{} = {}",
        x,
        y,
        z,
        multiplicador(x, y, z)
    );
}
