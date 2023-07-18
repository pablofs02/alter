use discus::config::cargar;
use discus::hacer_copia_de_seguridad;

fn main() {
    let directorios = cargar();
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).is_some() {
        for (origen, destino) in directorios {
            hacer_copia_de_seguridad(&destino, &origen);
        }
    } else {
        for (origen, destino) in directorios {
            hacer_copia_de_seguridad(&origen, &destino);
        }
    }
}
