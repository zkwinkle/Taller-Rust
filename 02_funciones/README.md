# Funciones

Las funciones se declaran con `fn`, como el `fn main()`.
En Rust es convencional usar *snake_case* para las funciones.

Así se escribe una función que toma un número y lo imprime:

```rust
fn imprimir_num(x: i32) {
    println!("Número: {}", x);
}
```

Los parámetros se declaran dentro del paréntesis con el format `(nombre: tipo)`.

Por ejemplo una función con que toma 3 números y los suma:

```rust
fn sumar_3_nums(x: i32, y: i32, z: i32) -> i32 {
    return (x + y + z);
}
```

Como se observa en la función anterior se declara el tipo de retorno con una flecha de la manera `-> tipo`.

También, no es necesario escribir `return` si se quiere retornar la última expresión en la función, para esto además no se pone el `;` al final. Ejemplo:
```rust
fn sumar_3_nums(x: i32, y: i32, z: i32) -> i32 {
    x + y + z
}
```

Es más convencional escribir las funciones de esta manera.

## Tarea

Arregle el código en `main.rs` para que compile, finalmente escriba una función llamada `multiplicador` que tome 3 números `x`, `y`, `z` y retorne su multiplicación.

## Más información

- [How Functions Work](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
