use disco::actualizar_directorio;
use std::collections::HashMap;

fn main() {
    let directorios = cargar_diccionario();
    for (c, v) in directorios {
        actualizar_directorio(c, v);
    }
}

fn cargar_diccionario() -> HashMap<String, String> {
    let mut d = HashMap::new();
    d.insert(String::from("/home/c1"), String::from("/mnt/c1"));
    d
}
