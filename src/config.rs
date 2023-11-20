use crate::variable::cambiar_variables;
use std::collections::HashMap;
use std::env::var;
use std::fs::read_to_string;

/// Genera un diccionario de copias a partir de un archivo de configuración
///
/// Recoge el archivo definido en la variable ~$ALTER~
/// El archivo por defecto se encuentra en /home/---/.config/alter
#[must_use]
pub fn cargar_modulus(disco: &str, modulus: Option<&str>) -> HashMap<String, String> {
    // ¡BUSCAR SOLO EN EL MODULUS INDICADO!
    let mut directorios = HashMap::new();
    let home = var("HOME").unwrap();
    let mut origen = String::from(&home);
    let mut destino: String = String::from(disco);
    let ruta_archivo_conf = var("ALTER").unwrap_or(format!("{home}/.config/alter"));
    let archivo_conf = read_to_string(ruta_archivo_conf).expect("Error al cargar configuración");
    // Buscar el modulus elegido
    if modulus.is_some() {
        for norma in archivo_conf.lines() {
            if norma.starts_with('[') && norma.ends_with(']') {
                let contenido = &norma[1..(norma.len() - 1)];
                println!("{:?}", contenido);
                //     origen = if contenido[0].starts_with('/') {
                //         String::from(contenido[0].trim())
                //     } else {
                //         format!("{home}/{}", contenido[0].trim())
                //     };
                //     destino = if let Some(cont) = contenido.get(1) {
                //         format!("{disco}/{}", cont.trim())
                //     } else {
                //         format!("{disco}")
                //     };
                continue;
            }
        }
    }
    // Sacar las transmutaciones del modulus
    for norma in archivo_conf.lines() {
        if norma.is_empty() || norma.starts_with('#') {
            continue;
        }
        let regla = cambiar_variables(&norma.replace('~', &home));
        if regla.starts_with('-') {
            let contenido: Vec<&str> = regla[1..].trim().split("->").collect();
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
        if regla.starts_with('[') && regla.ends_with(']') {
            break;
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
