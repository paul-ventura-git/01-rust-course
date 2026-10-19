pub struct ResultadoNumerico {
    pub nombre: String,
    pub total: i32,
}

pub fn ejecutar() {
    println!("\n--- MODULO 1: NUMEROS ---");

    let numeros: [i32; 5] = [10, 20, 30, 40, 50];
    let mut suma: i32 = 0;

    for numero in numeros {
        suma += numero;
    }

    let resultado = ResultadoNumerico {
        nombre: String::from("Suma de numeros"),
        total: suma,
    };
}