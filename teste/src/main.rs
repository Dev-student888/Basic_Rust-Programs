use std::io;

fn main(){
    let mut s = String::new();

    io::stdin()
    .read_line(&mut s)
    .expect("Error reading String");

    println!();
    print!("Você digitou: {s}");
    println!("Número de caracteres digitados: {}", s.trim().len());
}
