use crate::cambios::hay_cambios_en_contenido;
use std::fs;
use std::os::unix;
use std::os::unix::fs::{symlink, MetadataExt};
use std::path::PathBuf;
use ultimus::terminal::es_output_de_terminal;

/// Copia recursivamente un directorio origen en un directorio destino.
///
/// Solo copia los archivos que:
/// - No existen en el destino
/// - No están actualizados (fecha modificación)
pub fn hacer_copia_de_seguridad(dir_orig: &String, dir_dest: &String) {
    fs::symlink_metadata(dir_orig).map_or_else(
        |_| {
            if es_output_de_terminal() {
                eprintln!("No existe el archivo: \x1b[31m{dir_orig}\x1b[m");
            } else {
                eprintln!("No existe el archivo: {dir_orig}");
            }
        },
        |tipo| {
            if tipo.is_dir() {
                if fs::File::open(dir_orig).is_ok() {
                    fs::File::open(dir_dest).map_or_else(
                        |_| {
                            copiar_directorio(dir_orig, dir_dest);
                            informar(dir_orig, dir_dest);
                        },
                        |_| {
                            let subdirs = fs::read_dir(dir_orig).unwrap();
                            for subdir in subdirs {
                                let subdir = subdir.unwrap();
                                let carpeta = subdir.file_name().to_str().unwrap().to_string();
                                let ruta = subdir.path().to_str().unwrap().to_string();
                                hacer_copia_de_seguridad(&ruta, &format!("{dir_dest}/{carpeta}"));
                            }
                        },
                    );
                }
            } else if tipo.is_file() {
                if let Ok(orig) = fs::File::open(dir_orig) {
                    fs::File::open(dir_dest).map_or_else(
                        |_| {
                            let ruta = PathBuf::from(dir_dest);
                            let ruta = ruta.parent().unwrap().to_str().unwrap();
                            fs::create_dir_all(ruta).expect(ruta);
                            copiar_archivo(dir_orig, dir_dest);
                            informar(dir_orig, dir_dest);
                        },
                        |dest| {
                            if hay_cambios_en_contenido(&orig, &dest) {
                                copiar_archivo(dir_orig, dir_dest);
                                informar(dir_orig, dir_dest);
                            }
                        },
                    );
                }
            } else if tipo.is_symlink() {
                let enlace = fs::read_link(dir_orig).unwrap();
                if let Ok(puntero) = fs::read_link(dir_dest) {
                    if puntero != enlace {
                        fs::remove_file(dir_dest).unwrap();
                        symlink(enlace, dir_dest).unwrap();
                        informar(dir_orig, dir_dest);
                    }
                } else {
                    symlink(enlace, dir_dest).unwrap();
                    informar(dir_orig, dir_dest);
                }
            }
        },
    );
}

fn copiar_directorio(dir_orig: &String, dir_dest: &String) {
    if  dir_orig.ends_with("target") {
        return
    }
    fs::create_dir_all(dir_dest).expect(dir_dest);
    let subdirectorios = fs::read_dir(dir_orig).expect(dir_orig);
    for subdir in subdirectorios {
        let ruta = subdir.unwrap().path();
        if let Ok(tipo) = fs::symlink_metadata(&ruta) {
            let dir = ruta.file_name().unwrap().to_str().unwrap();
            let dir_dest = format!("{dir_dest}/{dir}");
            if tipo.is_dir() {
                copiar_directorio(&ruta.to_str().unwrap().to_string(), &dir_dest);
            } else if tipo.is_file() {
                let ruta = ruta.to_str().unwrap().to_string();
                copiar_archivo(&ruta, &dir_dest);
            } else if tipo.is_symlink() {
                let enlace = fs::read_link(&ruta).unwrap();
                symlink(&enlace, &dir_dest).unwrap();
            }
        }
    }
}

fn copiar_archivo(dir_orig: &str, dir_dest: &str) {
    if fs::copy(dir_orig, dir_dest).is_err() {
        eprintln!("\x1b[31mError al copiar el archivo: {dir_orig} -> {dir_orig}\x1b[m");
    };
    let metadata = fs::metadata(&dir_orig).unwrap();
    if unix::fs::chown(&dir_dest, Some(metadata.uid()), Some(metadata.gid())).is_err() {
        eprintln!("No se pudo cambiar el dueño");
    };
}

fn informar(dir_orig: &str, dir_dest: &str) {
    if !dir_orig.contains(".git") {
        if es_output_de_terminal() {
            println!("{dir_orig} \x1b[31m->\x1b[m {dir_dest}");
        } else {
            println!("{dir_orig} -> {dir_dest}");
        }
    }
}
