// Sus tareas son:
//
// 1. Arreglar el código para que compile.
//
// 2. Intenta eliminar el punto y coma en la línea marcada con XXX ;
// ¿qué cambios ocurren?
//
// 3. Vaya a la línea 56 y complete el ejercicio 3.

// Una estructura es un registro con nombre que contiene campos, similar a struct en C o class en Java.
struct Thing {
    // Sintaxis `campo: tipo` (como en las firmas de funciones, los tipos son obligatorios)
    label: char,
    count: i32,
    //     ^~~ `i`nteger de 32 bits con signo.
}

pub fn main() {
    let i = 5;

    //       V~~~~~~~~~ expresión de creación de estructura
    let t = Thing {
        label: 'a',
        count: i,
    };

    // Observa que `t` está *asignado en la pila*, al igual que `i` arriba;
    // al igual que en C/C++.
    //
    // (Compara con, por ejemplo, Java/Scheme/ML, donde las estructuras de registros
    // están asignadas en el montón y tus variables solo pueden contener *referencias*
    // a ellas).

    print_thing(t); // XXX (ver ejercicio 2)
}

fn print_thing(x: Thing) -> i32 {
    //         ^~~~~~~~ nuevamente, `x` está asignado en la pila localmente aquí.
    //                  Por lo tanto, este objeto ha sido *copiado* desde `main`
    //                  hasta aquí.

    // El acceso a los campos utiliza el operador punto, al igual que en C/Java.

    println!("el conteo de {:x} en hexadecimal es {:x}", x.label, x.count);
    //                     ^~~~           ^~~~
    // Los argumentos deben coincidir con los huecos (en número y especificador si está presente).

    // (devolviendo x.count sin razón real)
    x.count
}

#[cfg(test)]
mod tests {
    use super::*;

    // EJERCICIO 3: Complete lo que dicen los comentarios marcados con TODO para
    // que las pruebas compilen y pasen

    struct ColorClassicStruct {
        // TODO: Aquí va algo
    }

    struct ColorTupleStruct(/* TODO: Aquí va algo */);

    #[derive(Debug)]
    struct UnitStruct;

    #[test]
    fn classic_c_structs() {
        // TODO: Instanciar struct clásico de C!
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instanciar tuple struct!
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instanciar unit struct!
        // let unit_struct =
        let message = format!("{:?}s son divertidos!", unit_struct);

        assert_eq!(message, "UnitStructs son divertidos!");
    }
}
