// Supongamos que estamos escribiendo un juego en el que puedes comprar objetos con fichas. Todos los objetos cuestan
// 5 fichas, y siempre hay una tarifa de procesamiento de 1 ficha cada vez que compras objetos. Un jugador del juego
// escribirá la cantidad de objetos que desea comprar, y la función `total_cost` calculará el costo total de los objetos.
// Dado que el jugador escribió la cantidad como texto, sin embargo, la recibimos como una cadena de texto, y podrían haber
// escrito cualquier cosa, no solo números.
//
// En este momento, esta función no maneja el caso de error en absoluto (y tampoco maneja correctamente el caso de éxito).
// Lo que queremos hacer es lo siguiente: si llamamos a la función `total_cost` con una cadena que no es un número, esa
// función devolverá un `ParseIntError`, y en ese caso, queremos devolver ese error inmediatamente desde nuestra función y
// no intentar multiplicar y sumar.
//
// Hay al menos dos formas de implementar esto que son ambas correctas, ¡pero una es mucho más corta!

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
