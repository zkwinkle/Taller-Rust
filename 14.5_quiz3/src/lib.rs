// Quiz 3
//
// Este cuestionario evalúa:
// - Genéricos
// - Traits
//
// ¡Una escuela mágica imaginaria tiene un nuevo sistema de generación de boletines escrito
// en Rust! Actualmente, el sistema solo admite la creación de boletines donde la calificación
// del estudiante se representa numéricamente (por ejemplo, 1.0 -> 5.5). Sin embargo, la
// escuela también emite calificaciones alfabéticas (A+ -> F-) y debe poder imprimir ambos tipos
// de boletines.
//
// Realiza los cambios de código necesarios en la estructura ReportCard y el bloque impl
// para admitir boletines alfabéticos. Cambia el grado en la segunda prueba a "A+" para mostrar que
// tus cambios permiten calificaciones alfabéticas.

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{} ({}) - logró una calificación de {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - logró una calificación de 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Asegúrate de cambiar la calificación aquí después de terminar el ejercicio.
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - logró una calificación de A+"
        );
    }
}

