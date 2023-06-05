use disco::actualizar_directorio;
use disco::conf::cargar;

fn main() {
    let directorios = cargar();
    for (c, v) in directorios {
        actualizar_directorio(&c, &v);
    }
}
