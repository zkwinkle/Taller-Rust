// Quiz 1
//
// Este es un cuestionario para las siguientes secciones:
// - Variables
// - Funciones
// - If
//
// Mary está comprando manzanas. El precio de una manzana se calcula de la siguiente manera:
// - Una manzana cuesta 2 rustbucks.
// - ¡Si Mary compra más de 40 manzanas, cada manzana solo cuesta 1 rustbuck!
// Escribe una función que calcule el precio de un pedido de manzanas dado el
// cantidad comprada.

// ¡Coloca tu función aquí!
// fn calculate_price_of_apples {

// ¡No modifiques esta función!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
