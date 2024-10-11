extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Jogo da adivinhação");

   loop {
   let mut fase = 1;
   let mut points = 10;

    loop {
        let limite = fase * 100;
    
    println!("Fase {}: Adivinhe um número entre 1 e {}", fase, limite);
    println!("Você tem {} chances", points);    
    
    let numero_secreto = rand::thread_rng().gen_range(1, limite);
   
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

        if points <= 0 {
            println!("Game Over, você perdeu");
            break;
        }
    

    }
    if points <= 0 {
        break;
    }

    fase += 1;
    points += 5;
    println!("Parabéns! Você passou para a fase {} .\n", fase);


  }
  println!("Reiniciando o jogo...\n");

}
}
