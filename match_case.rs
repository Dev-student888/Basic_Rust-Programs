/* USAMOS O `match` CASE PARA ENCONTRAR UM VALOR CORRESPONDENTE A UM TIPO DE DADO
   A SINTAXE DA ESTRUTURA `match` CASE É A SEGUINTE:
        match nome_da_variavel { nome_da_variavel REFERE-SE AO PRÓPRIO NOME DA VARIÁVEL ESCOLHIDA
            "opção01" => RESPOSTA PARA A OPÇÃO 1; EX: println!() DEPOIS VEM A VÍRGULA `,` SEM ASPAS
            "opção02" => RESPOSTA PARA A OPÇÃO 2,
            "..." => PODE COLOCAR QUANTAS OPÇÕES QUISER,
            _ => RESPOSTA PARA A OPÇÃO `default` DO `match` CASE. _ É COMO SE FOSSE O `default` DO `match` CASE
        } 
*/

fn main(){
    let fruta: &str = "manga";

    match fruta { 
        "manga" => println!("Eu tenho uma {fruta} nas mãos"), // Nesta opção colocamos "manga" como alternativa
        "banana" => println!("Eu tenho uma {fruta} nas mãos"), // Nesta opção colocamos "banana" como alternativa
        "pêra" => println!("Eu tenho uma {fruta} nas mãos"), // ...
        "goiaba" => println!("Eu tenho uma {fruta} nas mãos"),
        "maçã" => println!("Eu tenho uma {fruta} nas mãos"),
        _ => println!("Esta fruta não está na lista!") // _ é como se fosse o default do `match` case
    }
}
