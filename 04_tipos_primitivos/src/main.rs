fn main() {
    /**** Booleanoss (`bool`) ****/

    let is_morning = true;
    if is_morning {
        println!("Buenos días!");
    }

    let // Termine la línea como el ejemplo, o hágalo falso!
    if is_evening {
        println!("Buenas noches!");
    }


    /**** Caracteres (`char`) ****/

    // Note el uso de comillas _simples_ ('), es diferente de las doble comillas
    // (") que hemos visto hasta el momento.
    let my_first_initial = 'I';
    if my_first_initial.is_alphabetic() {
        println!("Alfabética!");
    } else if my_first_initial.is_numeric() {
        println!("Numérica!");
    } else {
        println!("Ni alfabética ni numérica!");
    }

    let // Termine esta línea como  el ejemplo! Cuál es su caracter favorito?
    // Pruebe una letra, pruebe un número, pruebe un character especial, de otro
    // idioma, hasta un emoji!
    if your_character.is_alphabetic() {
        println!("Alfabética!");
    } else if your_character.is_numeric() {
        println!("Numérica!");
    } else {
        println!("Ni alfabética ni numérica!");
    }

    /**** Arrays (`[]`) ****/
    let a = ??? // Cree un array de al menos 20 elementos donde está el ???

    if a.len() >= 20 {
        println!("Wow, qué array más grande!");
    } else {
        println!("Meh, yo desayuno arrays de ese tamaño.");
        panic!("Array no es lo suficientemente grande")
    }

    /**** Tuplas (`()`) ****/
    let cat = ("Furry McFurson", 3.5);
    let /* Destructure la tupla para que el print de abajo funcione */ = cat;

    println!("{} is {} years old.", name, age);
}

#[test]
fn slice_out_of_array() {
    /**** Slices (son como pedazos de un array []) ****/
    let a = [1, 2, 3, 4, 5];

    let nice_slice = ??? // Tome un slice del array 'a' para que este test pase

    assert_eq!([2, 3, 4], nice_slice)
}

#[test]
fn indexing_tuple() {
    /**** Tuplas (`()`) ****/
    let numbers = (1, 2, 3);
    // Reemplace ??? con la expresión de índice de la tupla para tomar el
    // segundo elemento
    let second = ???;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
