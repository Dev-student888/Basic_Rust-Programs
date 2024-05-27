fn main() {

    /* EXISTEM TRÊS TIPOS DE OPERADORES LÓGICOS EM RUST:
       - E => &&
       - OU => ||
       - NÃO => !
    */

    let salario :usize = 1_200;
    if !{salario < 1200}{
        println!("Seu salário é maior ou igual a R$1200.00 ");
    } else{
        println!("Seu salário é inferior a R$1200.00 ");
    }
}
