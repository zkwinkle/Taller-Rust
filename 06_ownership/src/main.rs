// Tu tareas son:
//
// 1. Agregar código al final del main para imprimir tanto `t1` como `t2`
//    nuevamente. Discutir.
//
// 2. Agregar una nueva función, `increment_count`, que toma un
//    `Thing` y le suma 1 a su campo de recuento de una manera que el
//    *llamador* puede observar.
//
// 3. Haga que la prueba `solo_reordenar` compile y pase SOLO cambiando el orden
//    de las líneas, sin agregar, quitar ni modificar ninguna.

#[derive(Copy, Clone)]
struct Thing {
    label: char,
    count: i32,
}

fn print_thing_val(x: Thing) {
    println!("el recuento de {} es {}", x.label, x.count);
}

fn print_thing_ref(x: &Thing) {
    //                ^~ este es un "constructor de tipo de referencia"
    //                   `&T` se pronuncia "(referencia compartida) a T"
    println!("el recuento de `{}` es {}", x.label, x.count);

    let Thing { label, count } = *x;
    println!(
        "otra manera de vincular (`{}` sigue siendo {})",
        label, count
    );
}

fn print_thing_box(x: Box<Thing>) {
    //                      ^~~ Este es otro tipo de "referencia"
    //                          (Uno de los muchos "punteros inteligentes" proporcionados por la biblioteca)
    println!("el recuento de {} es {}", x.label, x.count);
}

pub fn main() {
    // asignado en la pila, como antes
    let t1 = Thing {
        label: 'a',
        count: 5,
    };

    // Estas dos llamadas tienen los mismos efectos secundarios ...
    print_thing_val(t1);
    print_thing_ref(&t1);
    // ... pero realizan su trabajo de maneras muy diferentes.

    // *Thing* asignado en el *heap*
    let t2 = Box::new(Thing {
        label: 'b',
        count: 4,
    });
    print_thing_box(t2);
}

// De qué maneras Box<Thing> se comporta diferente que Thing o &Thing? Discutan.

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    let z = &mut x;
    *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}
