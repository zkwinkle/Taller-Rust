// Puedes usar la palabra clave 'use' para traer rutas de módulos desde
// cualquier lugar y especialmente desde la biblioteca estándar de Rust a tu
// alcance. Trae SystemTime y UNIX_EPOCH del módulo std::time. ¡Puntos de estilo
// extra si puedes hacerlo con una sola línea!
use ???

// Has que el resto del código compilo


mod delicious_snacks;

fn main() {
    sausage_factory::make_sausage();


    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}

mod sausage_factory {
    // No deje que nadie afuera del módulo tenga acceso a esta función!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}
