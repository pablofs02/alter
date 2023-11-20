use std::collections::HashMap;
use std::env::var;
use std::fs::read_to_string;

/// Genera un diccionario de copias a partir de un archivo de configuración
///
/// Recoge el archivo definido en la variable ~$ALTER~
/// Solo recoge las transmutatio contenidas en el modulus especificado.
pub fn cargar_modulus_transmutatio(disco: &str, modulus: Option<&str>) -> HashMap<String, String> {
    let archivo_conf = cargar_archivo();
    let mut directorios = HashMap::new();
    let mut origen = String::from('.');
    let mut destino: String = String::from(disco);
    let mut normas_it = archivo_conf.lines();
    if modulus.is_some() {
        let mut normas = normas_it.next();
        while normas.is_some() {
            let norma = normas.unwrap();
            if norma.starts_with('[') && norma.ends_with(']') {
                let contenido = &norma[1..(norma.len() - 1)];
                if contenido == modulus.unwrap() {
                    break;
                }
            }
            normas = normas_it.next();
        }
    }
    for norma in normas_it {
        if norma.trim_start().is_empty() || norma.trim_start().starts_with('#') {
            continue;
        }
        if norma.starts_with('>') {
            let contenido: Vec<&str> = norma[1..].trim().split("->").collect();
            origen = if contenido[0].starts_with('/') {
                String::from(contenido[0].trim())
            } else {
                contenido[0].trim().to_owned()
            };
            destino = if let Some(cont) = contenido.get(1) {
                format!("{disco}/{}", cont.trim())
            } else {
                format!("{disco}")
            };
            continue;
        }
        if norma.starts_with('[') && norma.ends_with(']') {
            break;
        }
        let relatio: Vec<&str> = norma.split("->").collect();
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

fn cargar_archivo() -> String {
    let home = var("HOME").unwrap();
    if let Ok(config) = var("ALTER") {
        config
    } else if let Ok(config_home) = read_to_string(format!("{home}/.config/alter")) {
        config_home
    } else if let Ok(config_etc) = read_to_string("/etc/alter") {
        config_etc
    } else {
        eprintln!("No se encontró el archivo de configuración.");
        std::process::exit(1);
    }
}
