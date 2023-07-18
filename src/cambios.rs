use std::fs::File;
use std::io::{BufReader, Read};

pub fn hay_cambios_nuevos(orig: &File, dest: &File) -> bool {
    let meta_orig = orig.metadata().unwrap();
    let meta_dest = dest.metadata().unwrap();
    if meta_orig.modified().unwrap() < meta_dest.modified().unwrap() {
        return false;
    }
    if meta_orig.len() != meta_dest.len() {
        return true;
    }
    if meta_orig.modified().unwrap() == meta_dest.modified().unwrap() {
        return false;
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
