# Vectores

Los vectores son una de las estructuras de datos más utilizadas en Rust. En otros lenguajes de programación, simplemente se llamarían "Arrays", pero dado que Rust opera en un nivel un poco más bajo, un array en Rust se almacena en la pila (lo que significa que no puede crecer ni disminuir, y su tamaño debe conocerse en tiempo de compilación), mientras que un vector se almacena en el montón (donde estas restricciones no se aplican).

Los vectores son un tema un poco más avanzado en el libro, pero creo que son lo suficientemente útiles como para hablar de ellos un poco antes. Hablaremos sobre otra estructura de datos útil, los mapas de hash, más adelante.

## Tarea

Revise los archivos `main.rs` y `lib.rs`. 
Note que en este paquete se declaran 2 crates, uno ejecutable binario con el `main.rs` y el otro es una biblioteca con el `lib.rs`.

## Información adicional

- [Storing Lists of Values with Vectors](https://doc.rust-lang.org/stable/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)

