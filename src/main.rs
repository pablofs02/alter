fn main() {
    let argus: Vec<String> = std::env::args().collect();
    let disco = argus.get(1).unwrap_or_else(|| {
        mensaje_de_ayuda();
        std::process::exit(0);
    });
    let modulus = if let Some(modulus) = argus.get(2) {
        Some(modulus.as_str())
    } else {
        None
    };
    alter::realizar_copia_de_seguridad(disco, modulus);
}

fn mensaje_de_ayuda() {
    println!("Modo de empleo: alter <directorio-destino> [módulo]");
}
