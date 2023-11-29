// ¡Este es un ejercicio más grande que la mayoría de los demás! ¡Puedes hacerlo! Aquí está
// tu misión, si decides aceptarla:
// 1. Completa la función divide para que pasen los primeros cuatro tests.
// 2. Haz que pasen los tests restantes completando las funciones 
//    result_with_list y list_of_results.

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

// Calcula `a` dividido por `b` si `a` es divisble uniformemente por `b`.
// De lo contrario, devuelve un error adecuado.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    todo!();
}

// Completa la función y devuelve un valor del tipo correcto para que pase el test.
// Salida deseada: Ok([1, 11, 1426, 3])
fn result_with_list() -> () {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
}

// Completa la función y devuelve un valor del tipo correcto para que pase el test.
// Salida deseada: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> () {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
