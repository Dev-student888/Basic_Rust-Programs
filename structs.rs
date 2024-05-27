/* UMA STRUCT É UMA ESTRUTURA DE DADOS QUE NOS PERMITE ADICIONAR ATRIBUTOS A UM OBJETO.

   REGRAS PARA NOMENCLATURA DE STRUCTS:
   - DEVE COMEÇAR COM LETRA MAIÚSCULA

   NO CASO DE UMA STRUCT TER UM ATRIBUTO DO TIPO STRING, 
   DEVEMOS ATRIBUIR O VALOR A ESTE ATRIBUTO DESTA FORMA:
        atributo: String::from("nome_do_atributo");
*/

fn main(){

    let minha_moto = Moto{
        modelo: String::from("Honda"),
        ano: 1989,
        cor: String::from("Vermelha"),
        valor: 3200.00,
    };

    println!("Minha moto é uma {} {} {} que custou R${:.2} ", minha_moto.modelo, minha_moto.ano, minha_moto.cor, minha_moto.valor);
}

struct Moto{
    modelo: String,
    ano: u32,
    cor: String, 
    valor: f64,
}
