pub struct Mensaje {
    pub texto: String,
    pub longitud: usize,
}

pub fn ejecutar() {
    println!("\n--- MODULO 2: CADENAS ---");

    let nombres: [&str; 3] = ["Ana", "Carlos", "Maria"];

    for nombre in nombres {
        let mensaje = Mensaje {
            texto: String::from(nombre),
            longitud: nombre.len(),
        };

        if mensaje.longitud > 4 {
            println!(
                "{} tiene {} caracteres.",
                mensaje.texto, mensaje.longitud
            );
        } else {
            println!("{} tiene un nombre corto.", mensaje.texto);
        }
    }
}