use disco::conf::cargar;
use disco::hacer_copia_de_seguridad;

fn main() {
    let directorios = cargar();
    for (origen, destino) in directorios {
        hacer_copia_de_seguridad(&origen, &destino);
    }
}
