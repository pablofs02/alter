use std::collections::HashMap;
use std::env::var;
use std::fs::read_to_string;

#[must_use]
pub fn cargar() -> HashMap<String, String> {
    let mut directorios = HashMap::new();
    let home = var("HOME").unwrap_or_else(|_| "/home".to_owned());
    let mut disco = "/mnt".to_owned();
    let mut origen = String::from(&home);
    let mut destino: String = String::from(&disco);
    let ruta_archivo_conf = format!("{home}/.config/disco");
    let archivo_conf = read_to_string(ruta_archivo_conf).expect("Error al cargar configuración");
    for regla in archivo_conf.lines() {
        if regla.is_empty() {
            continue;
        }
        if regla.starts_with('[') && regla.ends_with(']') {
            if regla.starts_with("[[") && regla.ends_with("]]") {
                let contenido = &regla[2..(regla.len() - 2)];
                disco = String::from(contenido);
                continue;
            }
            let contenido: Vec<&str> = regla[1..(regla.len() - 1)].split("->").collect();
            origen = if contenido[0].starts_with('/') {
                String::from(contenido[0].trim())
            } else {
                format!("{home}/{}", contenido[0].trim())
            };
            if let Some(cont) = contenido.get(1) {
                destino = format!("{disco}/{}", cont.trim());
            } else {
                destino = disco.clone();
            }
            continue;
        }
        let relatio: Vec<&str> = regla.split("->").collect();
        let dir_base = if relatio[0].starts_with('/') {
            String::from(relatio[0].trim())
        } else {
            format!("{origen}/{}", relatio[0].trim())
        };
        let dir_copia = relatio.get(1).map_or_else(
            || format!("{destino}/{}", relatio[0].trim()),
            |cont| format!("{destino}/{}", cont.trim())
        );
        directorios.insert(dir_base, dir_copia);
    }
    directorios
}
