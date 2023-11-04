use crate::variable::cambiar_variables;
use std::collections::HashMap;
use std::env::var;
use std::fs::read_to_string;

/// Genera un diccionario de copias a partir de un archivo de configuración
///
/// Recoge el archivo definido en la variable ~$ALTER~
/// El archivo por defecto se encuentra en /home/---/.config/alter
#[must_use]
pub fn cargar(disco: &String) -> HashMap<String, String> {
    let mut directorios = HashMap::new();
    let home = var("HOME").unwrap();
    let mut origen = String::from(&home);
    let mut destino: String = String::from(disco);
    let ruta_archivo_conf = var("ALTER").unwrap_or(format!("{home}/.config/alter"));
    let archivo_conf = read_to_string(ruta_archivo_conf).expect("Error al cargar configuración");
    for regla in archivo_conf.lines() {
        if regla.is_empty() || regla.starts_with('#') {
            continue;
        }
        let regla = cambiar_variables(&regla.replace('~', &home));
        if regla.starts_with('[') && regla.ends_with(']') {
            let contenido: Vec<&str> = regla[1..(regla.len() - 1)].split("->").collect();
            origen = if contenido[0].starts_with('/') {
                String::from(contenido[0].trim())
            } else {
                format!("{home}/{}", contenido[0].trim())
            };
            destino = if let Some(cont) = contenido.get(1) {
                format!("{disco}/{}", cont.trim())
            } else {
                format!("{disco}")
            };
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
            |cont| format!("{destino}/{}", cont.trim()),
        );
        directorios.insert(dir_base, dir_copia);
    }
    directorios
}
