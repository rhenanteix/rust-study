extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Jogo da adivinhação");

   
   let mut fase = 1;
   let mut points = 10;
    loop {
        
    
    println!("Fase {}", fase);
    println!("Você tem {} chances", points);    
    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    loop {
        
        
        println!("Digite o seu palpite.");

        let mut  palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler a entrada");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_)=> continue,
        };

        
        println!("Você disse {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => {
                points -= 1;
                println!("Muito baixo! Perdeu ponto")  
            } 
            Ordering::Greater => {
                points -= 1;
                println!("Muito alto! Perdeu ponto")
            } 
            Ordering::Equal => {
             println!("Você acertou!!");
             break;
            }
        }

    } 
    fase += 1;
    println!("Parabéns! Você passou para a fase {} .\n", fase);

}
}
