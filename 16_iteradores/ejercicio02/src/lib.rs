// En este ejercicio, aprenderás algunas de las ventajas únicas que pueden ofrecer los iteradores.
// Sigue los pasos para completar el ejercicio.

// Paso 1.
// Completa la función `capitalize_first`.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => ???,
    }
}

// Paso 2.
// Aplica la función `capitalize_first` a una porción de cortes de cadenas.
// Devuelve un vector de cadenas.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    vec![]
}

// Paso 3.
// Aplica nuevamente la función `capitalize_first` a una porción de cortes de cadenas.
// Devuelve una sola cadena.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
