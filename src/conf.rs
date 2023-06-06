use std::collections::HashMap;
use std::fs::read_to_string;

pub fn cargar() -> HashMap<String, String> {
    let mut d = HashMap::new();
    let conf = read_to_string("/home/.config/disco").expect("Error al cargar configuración");
    for rel in conf.lines() {
        let rel: Vec<&str> = rel.split("=>").collect();
        let base = String::from(rel[0].trim()).replace("~/", "/home/");
        let copia = String::from(rel[1].trim()).replace("~/", "/home/");
        d.insert(base, copia);
    }
    d
}
