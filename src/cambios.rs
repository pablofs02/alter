use std::fs::File;
use std::io::{BufReader, Read};
use std::os::unix::fs::MetadataExt;

/// Verifica el haber cambios en el contenido de un archivo respecto a otro mirando:
/// - La fecha de modificación si es más reciente
/// - El tamaño de los archivos
/// - El contenido de los bytes (en bloques de 65536B)
pub fn hay_cambios_en_contenido(orig: &File, dest: &File) -> bool {
    let meta_orig = orig.metadata().unwrap();
    let meta_dest = dest.metadata().unwrap();
    if meta_orig.modified().unwrap() <= meta_dest.modified().unwrap() {
        return false;
    }
    if meta_orig.len() != meta_dest.len() {
        return true;
    }
    let mut lector_orig = BufReader::new(orig);
    let mut lector_dest = BufReader::new(dest);
    let mut buf_orig = [0; 65536];
    let mut buf_dest = [0; 65536];
    while let Ok(_) = lector_orig.read(&mut buf_orig) {
        if let Ok(_) = lector_dest.read(&mut buf_dest) {
            if buf_orig == buf_dest {
                continue;
            }
            return true;
        }
    }
    false
}

/// Mira los cambios que haya en:
/// - Permisos
/// - Propietario
/// - Grupo
pub fn hay_cambios_en_metadata(orig: &File, dest: &File) -> bool {
    let metadatos_orig = orig.metadata().unwrap();
    //let metadata = fs::metadata(&dir_orig).unwrap();
    //if unix::fs::chown(&dir_dest, Some(metadata.uid()), Some(metadata.gid())).is_err() {
    //eprintln!("No se pudo cambiar el dueño");
    //};
    let metadatos_dest = dest.metadata().unwrap();
    if metadatos_orig.mode() != metadatos_dest.mode() {
        return true;
    }
    false
}
