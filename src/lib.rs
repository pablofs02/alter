pub mod conf;
mod var;
use std::fs::{copy, read_dir, read_link, symlink_metadata, File};
use std::io::{BufReader, Read};
use std::os::unix::fs::symlink;
use std::path::PathBuf;

pub fn hacer_copia_de_seguridad(dir_base: &String, dir_copia: &String) {
    symlink_metadata(dir_base).map_or_else(
        |_| {
            eprintln!("No existe un archivo que se intenta copiar: {dir_base}");
        },
        |tipo| {
            if tipo.is_dir() {
                if File::open(dir_base).is_ok() {
                    File::open(dir_copia).map_or_else(
                        |_| {
                            copiar_directorio(dir_base, dir_copia);
                            println!("{dir_base} -> {dir_copia}");
                        },
                        |_| {
                            let subdirs = read_dir(dir_base).unwrap();
                            for subdir in subdirs {
                                let subdir = subdir.unwrap();
                                let carpeta = subdir.file_name().to_str().unwrap().to_string();
                                let ruta = subdir.path().to_str().unwrap().to_string();
                                hacer_copia_de_seguridad(&ruta, &format!("{dir_copia}/{carpeta}"));
                            }
                        },
                    );
                }
            } else if tipo.is_file() {
                if let Ok(orig) = File::open(dir_base) {
                    File::open(dir_copia).map_or_else(
                        |_| {
                            let ruta = PathBuf::from(dir_copia);
                            let ruta = ruta.parent().unwrap().to_str().unwrap();
                            std::fs::create_dir_all(ruta).expect(&ruta);
                            copy(dir_base, dir_copia)
                                .unwrap_or_else(|_| panic!("{dir_base}>{dir_copia}"));
                            println!("{dir_base} -> {dir_copia}");
                        },
                        |dest| {
                            if hay_cambios_nuevos(&orig, &dest) {
                                copy(dir_base, dir_copia)
                                    .unwrap_or_else(|_| panic!("{dir_base}>{dir_copia}"));
                                println!("{dir_base} -> {dir_copia}");
                            }
                        },
                    );
                }
            } else if tipo.is_symlink() {
                let enlace = read_link(dir_base).unwrap();
                if let Ok(puntero) = read_link(dir_copia) {
                    if puntero != enlace {
                        std::fs::remove_file(dir_copia).unwrap();
                        symlink(enlace, dir_copia).unwrap();
                        println!("{dir_base} -> {dir_copia}");
                    }
                } else {
                    symlink(enlace, dir_copia).unwrap();
                    println!("{dir_base} -> {dir_copia}");
                }
            }
        },
    );
}

fn copiar_directorio(dir_base: &String, dir_copia: &String) {
    std::fs::create_dir_all(dir_copia).expect(dir_copia);
    let subdirectorios = read_dir(dir_base).expect(dir_base);
    for subdir in subdirectorios {
        let ruta = subdir.unwrap().path();
        if let Ok(tipo) = symlink_metadata(&ruta) {
            let dir = ruta.file_name().unwrap().to_str().unwrap();
            let dir_dest = format!("{dir_copia}/{dir}");
            if tipo.is_dir() {
                copiar_directorio(&ruta.to_str().unwrap().to_string(), &dir_dest);
            } else if tipo.is_file() {
                let ruta = ruta.to_str().unwrap().to_string();
                copy(&ruta, &dir_dest).expect(&ruta);
            } else if tipo.is_symlink() {
                let enlace = read_link(&ruta).unwrap();
                symlink(&enlace, &dir_dest).unwrap();
            }
        }
    }
}

fn hay_cambios_nuevos(orig: &File, dest: &File) -> bool {
    if orig.metadata().unwrap().modified().unwrap() < dest.metadata().unwrap().modified().unwrap() {
        return false;
    }
    if orig.metadata().unwrap().len() != dest.metadata().unwrap().len() {
        return true;
    } else {
        if orig.metadata().unwrap().modified().unwrap() == dest.metadata().unwrap().modified().unwrap() {
            return false;
        }
    }
    let mut lector_orig = BufReader::new(orig);
    let mut lector_dest = BufReader::new(dest);
    let mut buf_orig = [0; 10000];
    let mut buf_dest = [0; 10000];
    while let Ok(n1) = lector_orig.read(&mut buf_orig) {
        if n1 == 0 {
            break;
        }
        if let Ok(n2) = lector_dest.read(&mut buf_dest) {
            if n1 == n2 && buf_orig == buf_dest {
                continue;
            }
            return true;
        }
    }
    false
}
