pub struct Producto {
    pub nombre: String,
    pub precio: i32,
    pub stock: i32,
}

pub fn ejecutar() {
    println!("\n--- MODULO 3: PRODUCTOS ---");

    let productos: [Producto; 3] = [
        Producto {
            nombre: String::from("Teclado"),
            precio: 80,
            stock: 10,
        },
        Producto {
            nombre: String::from("Mouse"),
            precio: 40,
            stock: 0,
        },
        Producto {
            nombre: String::from("Monitor"),
            precio: 700,
            stock: 5,
        },
    ];

    for producto in &productos {
        let valor_inventario = producto.precio * producto.stock;

        if producto.stock > 0 {
            println!(
                "{}: stock={}, valor={}",
                producto.nombre, producto.stock, valor_inventario
            );
        } else {
            println!("{}: sin stock.", producto.nombre);
        }
    }
}