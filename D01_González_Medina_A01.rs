use std::io; 

///Estructura que representa un producto con nombre, precio y cantidad
struct Producto {
    nombre: String, //Cadena que almacena el nombre del producto
    precio: f32, //Flotante que almacena el precio del producto
    cantidad: i32, //Entero que almacena la cantidad del producto
}

/// Función para agregar productos al carrito de compras 
/// Recibe una referencia mutable al vector carrito donde se almacenan los productos
fn agregar_producto(carrito : &mut Vec<Producto>) {
    let mut nombre = String::new(); //Variable mutable del nombre del producto
    let mut precio = String::new();  //Variable mutable del precio del producto
    let mut cantidad = String::new(); //Variable mutable de la cantidad del producto

    //Solicitar al usuario los datos del producto
    println!("\nIngrese el nombre del producto: ");
    io::stdin().read_line(&mut nombre).expect("Error al leer el nombre"); //Leer el nombre del producto
    println!("Ingrese el precio del producto: ");
    io::stdin().read_line(&mut precio).expect("Error al leer el precio"); //Leer el precio del producto
    println!("Ingrese la cantidad del producto: ");
    io::stdin().read_line(&mut cantidad).expect("Error al leer la cantidad"); //Leer la cantidad del producto

    // Convertir valores de entrada
    let precio: f32 = precio.trim().parse().expect("Precio inválido"); 
    let cantidad: i32 = cantidad.trim().parse().expect("Cantidad inválido"); 

    // Crear un producto con los datos ingresados
    let producto = Producto { 
        nombre: nombre.trim().to_string(),
        precio, 
        cantidad, 
    };
    carrito.push(producto);
}

/// Función para mostrar el carrito de compras y calcula el total
fn mostrar_carrito(carrito : &Vec<Producto>) {
    let mut total = 0.0; 
    println!("\nCarrito de compras: ");
    for producto in carrito { //Bucle para recorrer los productos del carrito
        println!("\nNombre: {}, Precio: {:.2}, Cantidad: {}", producto.nombre, producto.precio, producto.cantidad);
        total += producto.precio * producto.cantidad as f32; 
    }
    println!("\nTotal: {:.2}", total); 
}
/// Función principal que maneja las opciones del menú
fn main(){
    let mut carrito: Vec<Producto> = Vec::new(); 
    let mut opcion = String::new(); 

    loop{ //Bucle para mostrar el menú
        println!("\nBienvenido a la tienda de la esquina");
        println!("Menú: ");
        println!("1. Agregar producto al carrito");
        println!("2. Mostrar carrito de compras");
        println!("3. Salir");
        println!("\nIngrese una opción: ");

        opcion.clear(); //Limpia la variable de la opción
        io::stdin().read_line(&mut opcion).expect("Error al leer la opción"); //Lee la opción
        let opcion: i32 = opcion.trim().parse().expect("Opción inválida"); //Convierte la opción a entero y si es inválida muestra un mensaje

        match opcion { //Compara la opción ingresada
            1 => agregar_producto(&mut carrito), 
            2 => mostrar_carrito(&carrito), 
            3 => {  
                println!("Gracias por su compra");
                break; 
            }
            _ => println!("Opción no válida"), 
        }
    }
}