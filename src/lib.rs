use std::fs::{copy, File};
use std::io::{BufReader, Read};
use std::process::Command;

pub fn actualizar_directorio(dir_base: String, dir_copia: String) {
    let mut es_directorio: bool = false;
    let orig = File::open(&dir_base).unwrap(); // error si no hay origen
    if orig.metadata().unwrap().is_dir() {
        es_directorio = true;
    }
    if let Ok(dest) = File::open(&dir_copia) {
        if !es_directorio && hay_cambios(&orig, &dest) {
            copy(&dir_base, &dir_copia).unwrap();
            println!("{dir_base} => {dir_copia}");
        } else {
            match Command::new("cp")
                .arg("-r")
                .arg(&dir_base)
                .arg(&dir_copia)
                .output()
            {
                Ok(_) => println!("{dir_base} => {dir_copia}"),
                Err(_) => eprintln!("No se pudo copiar \"{dir_base}\" a \"{dir_copia}\"")
            }
        }
    } else {
        match Command::new("cp")
            .arg("-r")
            .arg(&dir_base)
            .arg(&dir_copia)
            .output()
        {
            Ok(_) => println!("{dir_base} => {dir_copia}"),
            Err(_) => eprintln!("No se pudo copiar \"{dir_base}\" a \"{dir_copia}\"")
        }
    }
}

fn hay_cambios(orig: &File, dest: &File) -> bool {
    if orig.metadata().unwrap().len() != dest.metadata().unwrap().len() {
        return true;
    }
    let mut lector_orig = BufReader::new(orig);
    let mut lector_dest = BufReader::new(dest);
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
