# Option

El tipo `Option` es otro de mis features favoritos de Rust, realmente no son un feature del lenguaje pero simplemente una parte de la biblioteca estándar que remplaza el concepto del `NULL` usando un enum. Representan un valor opcional: cada `Option` es o `Some` y contiene un valor, o `None`, y no contiene nada.
Los tipos `Option` son muy comunes en el código Rust, ya que tienen varios usos:

- Valores iniciales
- Valores de retorno para funciones que no están definidas en todo su rango de entrada (funciones parciales)
- Valor de retorno para informar sobre errores simples, donde None se devuelve en caso de error
- Campos opcionales en estructuras
- Campos de estructuras que pueden ser prestados o "tomados"
- Argumentos opcionales de funciones
- Punteros nulos
- Intercambiar cosas en situaciones difíciles

Algunos beneficios de usar `Option` en vez de tener un concepto como el NULL o None de Python es que:

- Siempre se sabe cuando un pedazo de código tiene la posibilidad de no retornar nada.
- El programador se ve obligado de lidiar con esta posibilidad.
- Evita el ["billion dollar mistake" del NULL](https://en.wikipedia.org/wiki/Void_safety#History).

## Tarea

Ver archivo `main.rs` y seguir comentarios para que compile y tests pasen y el código compile.

## Información Adicional

- [Option Enum Format](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
- [charla "Null References The Billion Dollar Mistake"](https://youtu.be/YYkOWzrO3xg?si=uULlVI9KcKIsqKIp)
