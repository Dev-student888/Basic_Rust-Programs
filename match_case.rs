fn main(){
    let fruta: &str = "manga";

    match fruta { // Usamos o `match` case para encontrar um valor correspondente a um tipo de dado
        "manga" => println!("Eu tenho uma {fruta} nas mãos"), // Nesta opção colocamos "manga" como alternativa
        "banana" => println!("Eu tenho uma {fruta} nas mãos"), // Nesta opção colocamos "banana" como alternativa
        "pêra" => println!("Eu tenho uma {fruta} nas mãos"), // ...
        "goiaba" => println!("Eu tenho uma {fruta} nas mãos"),
        "maçã" => println!("Eu tenho uma {fruta} nas mãos"),
        _ => println!("Esta fruta não está na lista!") // _ é como se fosse o default do `match` case
    }
}
