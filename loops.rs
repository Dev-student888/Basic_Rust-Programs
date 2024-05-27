/* EXISTEM TRÊS TIPOS DE LOOPS EM RUST:
   - O COMANDO FOR 
   - O COMANDO WHILE
   - O COMANDO LOOP
   
   OBS.: O COMANDO LOOP EXECUTA UM LOOP INFINITO!
   
   EXEMPLO DE COMANDO FOR:
        for i in 0..5{
            println!("Número: {}", i);
        } 

   EXEMPLO DE COMANDO WHILE:
        let mut x = 0;
        while x < 5{
            println!("X = {}", x);
            x = x + 1;
        }
    
   EXEMPLO DE COMANDO LOOP:
        let mut y = 0;
        loop{
            println!("Y = {} ", y);
            x = x + 1;
        }
*/

fn main(){

    let mut x = 0;
    while x < 5{
        println!("X = {} ", x);
        x = x + 1;
    }
}
