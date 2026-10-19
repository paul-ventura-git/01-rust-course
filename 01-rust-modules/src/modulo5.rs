pub struct Empleado {
    pub nombre: String,
    pub salario: i32,
}

fn calcular_bono(salario: i32) -> i32 {
    if salario >= 2000 {
        salario / 10
    } else {
        salario / 20
    }
}

pub fn ejecutar() {
    println!("\n--- MODULO 5: EMPLEADOS ---");

    let empleados: [Empleado; 3] = [
        Empleado {
            nombre: String::from("Lucia"),
            salario: 3000,
        },
        Empleado {
            nombre: String::from("Pedro"),
            salario: 1500,
        },
        Empleado {
            nombre: String::from("Sofia"),
            salario: 2500,
        },
    ];

    let mut gasto_total: i32 = 0;

    for empleado in &empleados {
        let bono = calcular_bono(empleado.salario);
        let pago_total = empleado.salario + bono;

        println!(
            "{}: salario={}, bono={}, total={}",
            empleado.nombre, empleado.salario, bono, pago_total
        );

        gasto_total += pago_total;
    }

    let resumen = format!("Gasto total de la empresa: {}", gasto_total);
    println!("{}", resumen);
}