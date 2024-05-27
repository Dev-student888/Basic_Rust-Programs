/* NO CASO DO RUST, NÃO SE USA PARÊNTESES PARA A ESTRUTURA CONDICIONAL IF-ELSE*/

fn main() {

    let salario = 1_200;

    if salario >= 1_100{
        println!("Seu salário é de: R${:.2}", salario);
    } else{
        println!("Seu salário é inferior a R$1100,00");
    }

}
