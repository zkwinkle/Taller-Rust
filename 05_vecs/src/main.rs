// Tu tarea es crear un `Vec` que contenga los mismos elementos que el
// array `a`.
//
// Haz que compile y pase la prueba.

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // un array simple
    let v = // TODO: declara tu vector aquí con la macro para vectores

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}

