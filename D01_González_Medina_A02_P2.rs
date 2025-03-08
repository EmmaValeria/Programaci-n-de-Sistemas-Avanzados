use std::io::{self, Write}; //Librerías necesarias

fn main(){
    let mut input = String::new(); //Variable mutable para la entrada del usuario
    print!("Ingrese el número de cadenas: "); //Solicta al usuario el número de cadenas
    io::stdout().flush().unwrap(); //Limpia el buffer de salida
    io::stdin().read_line(&mut input).unwrap(); //Lee la entrada del usuario

    let n: usize = input.trim().parse().unwrap(); //COnvierte la entrada a entero
    let mut cadenas: Vec<String> = Vec::with_capacity(n); //Vector de cadenas con capacidad n    

    for i in 0..n{ //Bucle para leer las cadenas
        let mut cadena = String::new(); //Variable mutable para la cadena
        print!("Ingrese la cadena {}: ", i+1); //Solicita al usuario la cadena
        io::stdout().flush().unwrap(); //Limpia el buffer de salida
        io::stdin().read_line(&mut cadena).unwrap(); //Lee la cadena
        cadenas.push(cadena.trim().to_string()); //Agrega la cadena al vector
    }

    println!("\nCadenas almacenadas en memoria: "); //Imprime las cadenas ingresadas
    for (i, cadena) in cadenas.iter().enumerate(){ //Bucle para recorrer las cadenas
        println!("Contenido de la cadena {}: {}", i+1, cadena); //Imprime la cadena
    }
}