pub mod conf;
use std::fs::{copy, read_dir, File};
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub fn actualizar_directorio(dir_base: &String, dir_copia: &String) {
    let mut es_directorio: bool = false;
    let orig = File::open(&dir_base).unwrap();
    if orig.metadata().unwrap().is_dir() {
        es_directorio = true;
    }
    if let Ok(dest) = File::open(&dir_copia) {
        if es_directorio {
            let subdirs = read_dir(&dir_base).unwrap();
            for subdir in subdirs {
                let subdir = subdir.unwrap();
                let carpeta = subdir.file_name().to_str().unwrap().to_string();
                let ruta = subdir.path().to_str().unwrap().to_string();
                actualizar_directorio(&ruta, &format!("{dir_copia}/{carpeta}"));
            }
        } else if hay_cambios(orig, dest) {
            copy(&dir_base, &dir_copia).unwrap();
            println!("{dir_base} => {dir_copia}");
        }
    } else {
        copiar_directorio(dir_base, dir_copia);
    }
}

fn copiar_directorio(dir_base: &String, dir_copia: &String) {
    std::fs::create_dir(&dir_copia).unwrap();
    let subdirectorios = read_dir(&dir_base).unwrap();
    for subdir in subdirectorios {
        let ruta = subdir.unwrap().path();
        let d = File::open(&ruta).unwrap();
        if d.metadata().unwrap().is_dir() {
            let dir = ruta.file_name().unwrap().to_str().unwrap();
            let dir_orig = ruta.to_str().unwrap().to_string();
            let dir_dest = format!("{dir_copia}/{dir}");
            println!("{dir_orig} => {dir_dest}");
            copiar_directorio(&ruta.to_str().unwrap().to_string(), &dir_dest);
        } else {
            copiar_archivo(ruta, &dir_copia);
        }
    }
}

fn copiar_archivo(dir_base: PathBuf, dir_copia: &String) {
    let arch = dir_base.file_name().unwrap().to_str().unwrap();
    let ruta = dir_base.to_str().unwrap().to_string();
    let ruta_dest = format!("{dir_copia}/{arch}");
    copy(&ruta, &ruta_dest).unwrap();
    println!("{ruta} => {ruta_dest}");
}

fn hay_cambios(orig: File, dest: File) -> bool {
    if orig.metadata().unwrap().len() != dest.metadata().unwrap().len() {
        return true;
    }
    let mut lector_orig = BufReader::new(&orig);
    let mut lector_dest = BufReader::new(&dest);
    let mut buf_orig = [0; 10000];
    let mut buf_dest = [0; 10000];
    loop {
        if let Ok(n1) = lector_orig.read(&mut buf_orig) {
            if n1 > 0 {
                if let Ok(n2) = lector_dest.read(&mut buf_dest) {
                    if n1 == n2 {
                        if buf_orig == buf_dest {
                            continue;
                        }
                    }
                    return true;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    false
}
