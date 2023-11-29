struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Arreglar sin cambiar ni eliminar esta línea
}

// Esta función devuelve cuánto helado queda en el refrigerador.
// Si es antes de las 10 PM, quedan 5 porciones. A las 10 PM, alguien se las come
// todas, ¡así que ya no quedará ninguna! :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // Utilizamos el sistema de 24 horas aquí, por lo que 10 PM es un valor de 22 y 12 AM es un
    // valor de 0. La salida Option debe manejar graciosamente los casos en que
    // hora_del_dia > 23.
    // TODO: Completa el cuerpo de la función - ¡recuerda devolver un Option!
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Arreglar este test. 
        // Cómo se obtiene el valor dentro del Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, 5);
    }

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Convierte esto en un `if let` cuyo valor es un tipo "Some"
        word = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: convierte esto en una declaración while let
        // recuerda que vector.pop también agrega otra capa de Option<T>.
        integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
