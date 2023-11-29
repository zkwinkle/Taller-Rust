# Manejo de errores

La mayoría de los errores no son lo suficientemente graves como para requerir que el programa se detenga por completo. A veces, cuando una función falla, es por una razón que se puede interpretar fácilmente y responder adecuadamente. Por ejemplo, si intentas abrir un archivo y la operación falla porque el archivo no existe, es posible que desees crear el archivo en lugar de terminar el proceso.

El manejo de errores en Rust es otra de mis features favoritas del lenguaje.
A diferencia de lenguajes como Python/C++/Java que "tiran excepciones" y todo
un sistema para manejarlas que confunde el flow del programa, en Rust simplemente
se tiene un enum en la biblioteca estándar llamado `Result`. Es muy similar
a `Option` y tiene beneficios parecidos como *siempre saber si un pedazo de
código tiene la capacidad de retornar error y obligar al programador a lididar con ellos.

## Tarea

Ver `main.rs` y módulos acompañantes y seguir instrucciones de comentarios. 

## Información adicional

- [Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Boxing errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)
