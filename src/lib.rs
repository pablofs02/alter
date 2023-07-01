pub mod conf;
use std::fs::{copy, read_dir, read_link, symlink_metadata, File};
use std::io::{BufReader, Read};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

pub fn hacer_copia_de_seguridad(dir_base: &String, dir_copia: &String) {
    if let Ok(tipo) = symlink_metadata(dir_base) {
        if tipo.is_dir() {
            if let Ok(_) = File::open(dir_base) {
                File::open(dir_copia).map_or_else(
                    |_| {
                        copiar_directorio(dir_base, dir_copia);
                    },
                    |_| {
                        let subdirs = read_dir(dir_base).unwrap();
                        for subdir in subdirs {
                            let subdir = subdir.unwrap();
                            let carpeta = subdir.file_name().to_str().unwrap().to_string();
                            let ruta = subdir.path().to_str().unwrap().to_string();
                            hacer_copia_de_seguridad(&ruta, &format!("{dir_copia}/{carpeta}"));
                        }
                    }
                );
            } else {
                println!("No existe un archivo que se intenta copiar: {dir_base}");
            }
        } else if tipo.is_file() {
            if let Ok(orig) = File::open(dir_base) {
                File::open(dir_copia).map_or_else(
                    |_| {
                        let ruta = PathBuf::from(dir_copia);
                        let ruta = ruta.parent().unwrap().to_str().unwrap();
                        std::fs::create_dir_all(ruta).unwrap();
                        copy(dir_base, dir_copia)
                            .unwrap_or_else(|_| panic!("{dir_base}>{dir_copia}"));
                    },
                    |dest| {
                        if hay_cambios_nuevos(&orig, &dest) {
                            copy(dir_base, dir_copia)
                                .unwrap_or_else(|_| panic!("{dir_base}>{dir_copia}"));
                            println!("{dir_base} -> {dir_copia}");
                        }
                    }
                );
            } else {
                println!("No existe un archivo que se intenta copiar: {dir_base}");
            }
        } else if tipo.is_symlink() {
            if let Ok(puntero) = read_link(dir_copia) {
                let enlace = read_link(dir_base).unwrap();
                if puntero != enlace {
                    std::fs::remove_file(dir_copia).unwrap();
                    let enlace = read_link(dir_base).unwrap();
                    symlink(enlace, dir_copia).unwrap();
                    println!("{dir_base} -> {dir_copia}");
                }
            } else {
                let enlace = read_link(dir_base).unwrap();
                symlink(enlace, dir_copia).unwrap();
                println!("{dir_base} -> {dir_copia}");
            }
        }
    }
}

fn copiar_directorio(dir_base: &String, dir_copia: &String) {
    std::fs::create_dir_all(dir_copia).expect(dir_copia);
    let subdirectorios = read_dir(dir_base).expect(dir_base);
    for subdir in subdirectorios {
        let ruta = subdir.unwrap().path();
        if let Ok(tipo) = symlink_metadata(&ruta) {
            if tipo.is_dir() {
                let dir = ruta.file_name().unwrap().to_str().unwrap();
                let dir_orig = ruta.to_str().unwrap().to_string();
                let dir_dest = format!("{dir_copia}/{dir}");
                println!("{dir_orig} -> {dir_dest}");
                copiar_directorio(&ruta.to_str().unwrap().to_string(), &dir_dest);
            } else if tipo.is_file() {
                copiar_archivo(&ruta, dir_copia);
            } else if tipo.is_symlink() {
                let enlace = read_link(dir_base).unwrap();
                symlink(enlace, dir_copia).unwrap();
                println!("{dir_base} -> {dir_copia}");
            }
        }
    }
}

fn copiar_archivo(dir_base: &Path, dir_copia: &str) {
    let arch = dir_base.file_name().unwrap().to_str().unwrap();
    let ruta = dir_base.to_str().unwrap().to_string();
    let ruta_dest = format!("{dir_copia}/{arch}");
    copy(&ruta, &ruta_dest).unwrap();
    println!("{ruta} -> {ruta_dest}");
}

fn hay_cambios_nuevos(orig: &File, dest: &File) -> bool {
    if orig.metadata().unwrap().modified().unwrap() < dest.metadata().unwrap().modified().unwrap() {
        return false;
    }
    if orig.metadata().unwrap().len() != dest.metadata().unwrap().len() {
        return true;
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
