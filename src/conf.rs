use std::collections::HashMap;
use std::fs::read_to_string;

pub fn cargar_diccionario() -> HashMap<String, String> {
    let mut d = HashMap::new();
    let conf = read_to_string("conf").expect("Error al cargar configuración");
    for rel in conf.lines() {
        let rel: Vec<&str> = rel.split("//").collect();
        d.insert(String::from(rel[0].trim()), String::from(rel[1].trim()));
    }
    d
}
