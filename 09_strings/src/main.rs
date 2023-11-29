// Hacer que el codigo compile y atender los TODOs para que las pruebas pasen

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);

    let word = String::from("green"); // Intenta no cambiar esta línea :)
    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }

    // TODO: Ok, aquí hay un montón de valores, algunos son `String`s, otros son
    // `&str`s. Su tarea es llamar a una de estas dos funciones en cada valor
    // dependiendo de lo que creas que es cada valor. Es decir, añade ya sea
    // `string_slice` o `string` antes de los paréntesis en cada línea. ¡Si a
    // ciertas, compilará!

    ???("blue");
    ???("red".to_string());
    ???(String::from("hi"));
    ???("rust is fun!".to_owned());
    ???("nice weather".into());
    ???(format!("Interpolación {}", "Estación"));
    ???(&String::from("abc")[0..1]);
    ???("  hello there ".trim());
    ???("¡Feliz lunes!".to_string().replace("Mon", "Martes"));
    ???("mY sHiFt KeY iS sTiCkY".to_lowercase());
}

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

// NO CAMBIAR LA DECLARACIÓN DE LA FUNCIÓN, solo el cuerpo
fn current_favorite_color() -> String {
    "blue"
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trim_me(input: &str) -> String {
        // TODO: ¡Eliminar los espacios en blanco de ambos extremos de una cadena!
        ???
    }

    fn compose_me(input: &str) -> String {
        // TODO: ¡Agregar " mundo!" a la cadena! ¡Hay varias formas de hacer esto!
        ???
    }

    fn replace_me(input: &str) -> String {
        // TODO: ¡Reemplazar "cars" en la cadena con "balloons"!
        ???
    }

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("¡Hola!     "), "¡Hola!");
        assert_eq!(trim_me("  ¿Qué pasa!"), "¿Qué pasa!");
        assert_eq!(trim_me("   ¡Hola!  "), "¡Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hola"), "Hola mundo!");
        assert_eq!(compose_me("Adiós"), "Adiós mundo!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("Creo que los autos son geniales"), "Creo que los globos son geniales");
        assert_eq!(replace_me("Me encanta mirar autos"), "Me encanta mirar globos");
    }
}
