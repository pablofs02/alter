enum Estado {
    Buscando,
    Tipificando,
    Variable,
    VariableCerrada,
}

pub fn cambiar_variables(cadena: &str) -> String {
    let mut respuesta = String::new();
    let mut variable = String::new();
    let mut estado = Estado::Buscando;
    for c in cadena.chars() {
        match estado {
            Estado::Buscando => {
                if c == '$' {
                    estado = Estado::Tipificando;
                } else {
                    respuesta.push(c);
                }
            }
            Estado::Tipificando => match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    variable.push(c);
                    estado = Estado::Variable;
                }
                '{' => estado = Estado::VariableCerrada,
                _ => {
                    respuesta.push('$');
                    respuesta.push(c);
                    estado = Estado::Buscando;
                }
            },
            Estado::Variable => match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    variable.push(c);
                }
                _ => {
                    respuesta.push_str(&tomar_variable(&variable));
                    respuesta.push(c);
                    estado = Estado::Buscando;
                }
            },
            Estado::VariableCerrada => match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                    variable.push(c);
                }
                '}' => {
                    respuesta.push_str(&tomar_variable(&variable));
                    estado = Estado::Buscando;
                }
                _ => {
                    respuesta.push_str(&variable);
                    respuesta.push(c);
                    estado = Estado::Buscando;
                }
            },
        }
    }
    respuesta.clone()
}

fn tomar_variable(cadena: &str) -> String {
    if let Ok(contenido) = std::env::var(cadena) {
        return contenido;
    }
    String::new()
}
