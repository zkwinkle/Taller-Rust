// ¡Este programa de lista de compras no se está compilando! Usa tu conocimiento de genéricos para arreglarlo.

fn main() {
    let mut shopping_list: Vec<?> = Vec::new();
    shopping_list.push("milk");
}

// Este "wrapper" poderoso proporciona la capacidad de almacenar un valor entero positivo.
// Reescríbelo utilizando genéricos para que admita envolver CUALQUIER tipo.

struct Wrapper {
    value: u32,
}

impl Wrapper {
    pub fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
