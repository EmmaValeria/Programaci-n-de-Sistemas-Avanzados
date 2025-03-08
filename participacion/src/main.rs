fn main(){
    //let x = 5;
      
      //x=10; Esto no es valido
  
      //let mut y = 10;
       //y = 20;
  
      let x = 5;
      { //Ambiente
          let x = x+1; //Nueva variable
          println!("EL valor de x es: {}", x);    
      }
      println!("EL valor de x es: {}", x);   
      println!("Hello, world");
      //Variables///
      let entero: i32 = 42;
      let flotante: f64 = 3.1416;
      let booleano: bool=true;
      let caracter: char = 'a';
  
      //tupla -> structs //creacion de tupla y arreglo
      let firulais: (i32, f64, char) = (43, 4.1416, 'b');
      let arreglo: [i32; 3] = [1,2,3];
      println!("Tupla (firulais) forma1: {:?}", firulais);
      println!("Tupla (firulais) forma2: ({}, {}, {})", firulais.0, firulais.1, firulais.2);
  
}
