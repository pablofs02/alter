use std::collections::HashMap;
use std::env::var;
use std::fs::read_to_string;

#[must_use]
pub fn cargar() -> HashMap<String, String> {
    let mut directorios = HashMap::new();
    let home = var("HOME").unwrap_or_else(|_| "/home".to_owned());
    let mut origen = String::from(&home);
    let mut destino = "/".to_owned();
    let ruta_archivo_conf = format!("{home}/.config/disco");
    let archivo_conf = read_to_string(ruta_archivo_conf).expect("Error al cargar configuración");
    for relatio in archivo_conf.lines() {
        let relatio = relatio.replace('~', &home[..]);
        if relatio.starts_with('[') && relatio.ends_with(']') {
            if relatio.starts_with("[[") && relatio.ends_with("]]") {
                let contenido = &relatio[2..(relatio.len() - 2)];
                destino = String::from(contenido);
            } else {
                let contenido = &relatio[1..(relatio.len() - 1)];
                origen = if contenido.starts_with('/') {
                    String::from(contenido)
                } else {
                    format!("{home}/{contenido}")
                };
            }
        } else {
            let relatio: Vec<&str> = relatio.split("=>").collect();
            let dir_base = if relatio[0].starts_with('/') {
                String::from(relatio[0].trim())
            } else {
                format!("{origen}/{}", relatio[0].trim())
            };
            let copia = String::from(relatio[1].trim());
            let dir_copia = format!("{destino}/{copia}");
            directorios.insert(dir_base, dir_copia);
        }
    }
    directorios
}
