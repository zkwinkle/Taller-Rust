// El compilador de Rust necesita saber cómo verificar si las referencias proporcionadas son válidas,
// para que pueda informar al programador si una referencia corre el riesgo de
// salir de ámbito antes de ser utilizada. Recuerda, las referencias son borrows y no
// son propietarias de sus propios datos. ¿Qué pasa si su propietario sale de ámbito?
//
// Has que este código compile

// Discuta cuántos parámetros de lifetime se ocupan para esta función
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Ejercicio 1
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz";

        result = longest(string1.as_str(), string2);
    }
    println!("La cadena más larga es '{}'", result);


    // Ejercicio 2
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}

// Los lifetimes también son necesarios cuando las estructuras contienen referencias.
struct Book {
    author: &str,
    title: &str,
}

