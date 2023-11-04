use alter::hacer_copia_de_seguridad;
use alter::opciones;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let disco = args.get(1).unwrap_or_else(|| {
        mensaje_de_ayuda();
        exit(0);
    });
    let directorios = opciones::cargar(disco);
    for (origen, destino) in directorios {
        hacer_copia_de_seguridad(&origen, &destino);
    }
}

fn mensaje_de_ayuda() {
    println!("Modo de empleo: alter <carpeta>");
}
