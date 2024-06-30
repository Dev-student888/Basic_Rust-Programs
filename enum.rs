/* ENUM É COMO SE FOSSE UM OBJETO QUE JÁ TIVESSE SEUS VALORES PRÉ-DETERMINADOS.
   PARA CRIAR UM `enum` NA LINGUAGEM RUST, DIGITAMOS A PALAVRA enum SEGUIDO DO 
   NOME DO RESPECTIVO ENUM SEGUIDO DE CHAVES. DENTRO DESSAS CHAVES COLOCAMOS AS 
   PROPRIEDADES OU OPÕES DESSE ENUM.
   EX: enum Animal {
           Cachorro, 
           Gato,
           Calopsita
       } 
*/

enum Animal {
    Gato,
    Cachorro,
    Hamster, 
    Calopsita, 
    Rato
}

fn main(){
    let meu_pet: Animal = Animal::Hamster;

    match meu_pet {
        Animal::Hamster => println!("Seu animal é um Hamster"),
        Animal::Gato => println!("Seu animal é um Gato"),
        Animal::Cachorro => println!("Seu animal é um Cachorro"),
        Animal::Calopsita => println!("Seu animal é uma Calopsita"),
        Animal::Rato => println!("Seu animal é um Rato"),
        _ => println!("Seu animal não está na lista")
    }
}
