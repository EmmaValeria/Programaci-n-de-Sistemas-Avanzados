use std::sync::{Arc, Mutex}; //Librería para sincronización de hilos
use std::thread; //Librería para hilos

const NUM_THREADS: usize = 15; //Número de hilos
const NUM_ITERATIONS: usize = 1000; //Número de iteraciones

fn main(){
    let counter = Arc::new(Mutex::new(0)); //Contador global compartido entre los hilos
    let mut threadswork = vec![]; //Vector para almacenar los hilos

    for _ in 0..NUM_THREADS { //Bucle para crear los hilos
        let counter = Arc::clone(&counter); //Clona el contador
        let task = thread::spawn(move || { //Crea un hilo
            for _ in 0..NUM_ITERATIONS { //Bucle para incrementar el contador
                let mut num = counter.lock().unwrap(); //Bloqueo del mutex
                *num += 1; //Incrementa el contador
            }
        });
        threadswork.push(task); //Agrega el hilo al vector
    }

    for task in threadswork { //Bucle para esperar a que los hilos terminen
        task.join().unwrap(); //Esperar a que el hilo termine
    }

    println!("Valor final del contador global: {}", *counter.lock().unwrap()); //Imprimir el valor del contador
}