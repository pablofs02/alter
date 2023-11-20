mod cambios;
mod config;
mod copia;

pub fn realizar_copia_de_seguridad(dir_dest: &str, modulus: Option<&str>) {
    let transmutatio = config::cargar_modulus_transmutatio(dir_dest, modulus);
    for (origen, destino) in transmutatio {
        copia::hacer_copia_de_seguridad(&origen, &destino);
    }
}
