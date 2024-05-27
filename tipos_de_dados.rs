fn main() {
    
    /* TIPOS DE DADOS EM RUST:
       - INTEIRO (INT) => i32, u32
       - DECIMAL (FLOAT) => f64, f32
       - BOOLEANO (BOOLEAN) => bool
       - CARACTERE (CHAR) -> CARACTERIZADO POR ASPAS SIMPLES => char
       - STRING -> CARACTERIZADO POR ASPAS DUPLAS => &str
    */

    let numero = 1_532; // Se não declarado o tipo de dado, o compilador assume que, 
    // se o dado for inteiro, ele será de 32 bits, ou seja, i32 ou u32.
    let decimal = 5_265.35;

    println!("numero = {} \ndecimal = {} ", numero, decimal);
}
