# Traits

Un trait es una colección de métodos.

Los tipos de datos pueden implementar traits. Para hacerlo, se definen los métodos que componen el trait para el tipo de datos. Por ejemplo, el tipo de dato `String` implementa el trait `From<&str>`. Esto permite a un usuario escribir `String::from("hello")`.

De esta manera, los traits son algo similares a las interfaces de Java y a las clases abstractas de C++.

Algunos traits comunes adicionales en Rust incluyen:

- `Clone` (el método `clone`)
- `Display` (que permite mostrar de manera formateada mediante `{}`)
- `Debug` (que permite mostrar de manera formateada mediante `{:?}`)

Dado que los traits indican un comportamiento compartido entre tipos de datos, son útiles al escribir genéricos.

## Tarea

Ver `main.rs` y el módulo acompañante y seguir instrucciones de comentarios. 

## Información adicional

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
