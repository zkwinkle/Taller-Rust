pub fn factorial(num: u64) -> u64 {
    // Completa esta función para devolver el factorial de num
    // No uses:
    // - return
    // Trata de no usar:
    // - bucles de estilo imperativo (for, while)
    // - variables adicionales
    // Para un desafío adicional, no uses:
    // - recursión
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_de_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_de_1() {
        assert_eq!(1, factorial(1));
    }

    #[test]
    fn factorial_de_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_de_3() {
        assert_eq!(6, factorial(3));
    }

    #[test]
    fn factorial_de_4() {
        assert_eq!(24, factorial(4));
    }

    #[test]
    fn factorial_de_5() {
        assert_eq!(120, factorial(5));
    }
}

