// Quiz 2
//
// Este es un quiz para las siguientes secciones:
// - Strings
// - Vectores (Vecs)
// - Ownership
// - Módulos
// - Enumeraciones (Enums)
//
// Construyamos una pequeña máquina en forma de función. Como entrada, daremos
// una lista de cadenas y comandos. Estos comandos determinan qué acción se
// aplicará a la cadena. Puede ser:
// - Convertir la cadena a mayúsculas
// - Recortar la cadena
// - Agregar "bar" a la cadena un número especificado de veces
// La forma exacta será:
// - La entrada va a ser un vector de una tupla de longitud 2,
//   el primer elemento es la cadena, el segundo es el comando.
// - El elemento de salida va a ser un vector de cadenas.

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: ¡Completa la firma de la función!
    pub fn transformer(input: ???) -> ??? {
        // TODO: ¡Completa la declaración de salida!
        let mut output: ??? = vec![];
        for (string, command) in input.iter() {
            // TODO: Completa el cuerpo de la función. ¡Puedes hacerlo!
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: ¿Qué necesitamos importar para tener `transformer` en el ámbito?
    use super::my_module::transformer as transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
