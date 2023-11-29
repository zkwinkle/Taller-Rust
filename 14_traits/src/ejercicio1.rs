// ¡Es hora de implementar algunos rasgos! Tu tarea es implementar el rasgo
// `AppendBar` para el tipo `String` y `Vec<String>`.
// El rasgo AppendBar tiene solo una función, que añade "Bar" a cualquier
// objeto que implemente este rasgo.

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implementa `AppendBar` para el tipo `String`.
}

// TODO: Implementa `AppendBar` para un vector de `String` ( Vec<String> ).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
