pub struct Estudiante {
    pub nombre: String,
    pub nota: i32,
}

pub fn ejecutar() {
    println!("\n--- MODULO 4: ESTUDIANTES ---");

    let notas: [i32; 4] = [15, 8, 18, 11];
    let mut aprobados: i32 = 0;

    for (indice, nota) in notas.iter().enumerate() {
        let estudiante = Estudiante {
            nombre: format!("Estudiante {}", indice + 1),
            nota: *nota,
        };

        if estudiante.nota >= 11 {
            println!(
                "{} aprobo con {}.",
                estudiante.nombre, estudiante.nota
            );
            aprobados += 1;
        } else {
            println!(
                "{} no aprobo: {}.",
                estudiante.nombre, estudiante.nota
            );
        }
    }

    println!("Total de aprobados: {}", aprobados);
}