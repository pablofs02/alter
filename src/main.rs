use disco::actualizar_directorio;
use disco::conf::cargar_diccionario;

fn main() {
    let directorios = cargar_diccionario();
    for (c, v) in directorios {
        actualizar_directorio(c, v);
    }
}
