// Esta función se niega a generar texto para imprimir en una etiqueta con nombre
// si le pasas una cadena vacía. Sería mejor si explicara cuál es el problema,
// en lugar de simplemente devolver `None` a veces. Afortunadamente, Rust tiene 
// una construcción (Result) similar a `Option` que se puede usar para expresar
// condiciones de error. ¡Usemos eso!

pub fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // Los nombres vacíos no están permitidos.
        None
    } else {
        Some(format!("¡Hola! Mi nombre es {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("¡Hola! Mi nombre es Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // No cambiar esta línea
            Err("`name` estaba vacío; debe ser no vacío.".into())
        );
    }
}
