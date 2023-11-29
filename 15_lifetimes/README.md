# Lifetimes

Los "lifetime"s le indican al compilador cómo verificar si las referencias tienen una duración suficiente para ser válidas en una situación dada. Por ejemplo, las lifetimes dicen "asegúrate de que el parámetro 'a' tenga una duración igual a la del parámetro 'b' para que el valor de retorno sea válido".

Solo son necesarias en préstamos, es decir, referencias, ya que los parámetros copiados o los movimientos son propiedad en su ámbito y no pueden ser referenciados externamente. Los "lifetime"s significan que el código que llama, por ejemplo, funciones, puede ser verificado para asegurarse de que sus argumentos sean válidos. Los lifetimes imponen restricciones a quienes las llaman.

Si deseas obtener más información sobre las anotaciones de "lifetime"s, el proyecto [lifetimekata](https://tfpk.github.io/lifetimekata/) tiene un estilo similar de ejercicios a estos, pero se centra en aprender a escribir anotaciones de lifetimes.

También se recomienda el capítulo del libro linkeado abajo, que es bastante más corto.

## Tarea

Ver archivo `main.rs`.

## Información adicional

- [Lifetimes (en Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
