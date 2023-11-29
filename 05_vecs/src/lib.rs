// Se proporciona un Vec de números pares. Tu tarea es completar el bucle para que
// cada número en el Vec se multiplique por 2.
//
// ¡Haz que pase la prueba!

// NO HE TERMINADO

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Rellena esto para que cada elemento en el Vec `v` sea
        // multiplicado por 2.
    }

    // En este punto, `v` debería ser igual a [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter()
        .map(|element| {
            // TODO: Haz lo mismo que arriba, pero en lugar de mutar el
            // Vec, ¡simplemente devuelve el nuevo número!
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
