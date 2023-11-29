// Al realizar operaciones en elementos dentro de una colección, los iteradores son
// esenciales. Este módulo te ayuda a familiarizarte con la estructura de uso de un
// iterador y cómo recorrer elementos dentro de una colección iterable.
//
// Haz que compile completando los `???`

#[test]
fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = ???;   // TODO: Paso 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Paso 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Paso 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), ???);     // TODO: Paso 4
}

