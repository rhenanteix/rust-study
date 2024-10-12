extern crate rand;
extern crate colored;

use std::{io::{self, Write}, thread::sleep, time::{Duration, Instant}};
use colored::Colorize;
use std::cmp::Ordering;
use rand::Rng;

fn print_for_animation(text: &str) {
    for c in text.chars() {
        print!("{}", c.to_string().bright_green());
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(100));
    }
    println!();
}

fn choice_nivel_animation(text: &str) {
    for u in text.chars() {
        print!("{}", u.to_string().bright_green());
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(100));
    }
    println!();
}


fn main() {
    print_for_animation("Bem vindo ao jogo da adivinhação");

    choice_nivel_animation("Escolha um nível de dificuldade");
    println!("{}","1- Fácil".blue());
    println!("{}", "2- Médio".purple());
    println!("{}", "3- Difícil".red());

    let mut modo = String::new();
    io::stdin().read_line(&mut modo).expect("Falha ao ler a entrada");
    let modo: u32 = modo.trim().parse().expect("Por favor, insira um número");

    let (vidas_iniciais, dicas_detalhadas) = match modo {
        1 => (15, true),
        2 => (10, false),
        3 => (5, false),
        _ => {
            println!("Modo inválido! Iniciando no modo médio por padrão.");
            (10, false) 
        }
    };

    println!("{}", "Você quer um jogo cronometrado? (s/n)".blue());

    let mut modo_cronometrado = String::new();
    io::stdin().read_line(&mut modo_cronometrado).expect("Erro ao ler entrada");

    let usar_cronometrado = modo_cronometrado.trim().to_lowercase() == "s";
    let tempo_limite = Duration::new(30, 0);

   loop {
   let mut fase = 1;
   let mut points = vidas_iniciais;


    loop {
       
    let limite = fase * 100;
    println!("Fase {}: Adivinhe um número entre 1 e {}", fase, limite);
    println!("Você tem {} chances", points);    
    
    let numero_secreto = rand::thread_rng().gen_range(1, limite);
   
     let inicio_fase = if usar_cronometrado {
         Some(Instant::now())
     } else {
        None
     };   

    loop {
        
        if usar_cronometrado {
            if let Some(inicio) = inicio_fase  {
                if inicio.elapsed() > tempo_limite {
                    println!("{}", "Tempo esgotado, você perdeu".red());
                    break;
                }
                println!("Tempo restante: {:.2} segundos", tempo_limite.as_secs_f64() - inicio.elapsed().as_secs_f64());
                
            }
        }
        
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
                
                if  dicas_detalhadas {
                    if (numero_secreto - palpite) > 50 {
                        println!("{}", "Muito baixo! Tente um número bem maior.".red());
                    } else if (numero_secreto - palpite) > 20 {
                        println!("{}", "Um pouco baixo! Tente um número maior.".red());
                    } else {
                        println!("{}", "Quase! Um pouco maior.".red());
                    }  
                } else {
                    println!("{}", "Muito baixo! Perdeu uma vida".red());  
                }
                
            } 
            Ordering::Greater => {
                points -= 1;
                if dicas_detalhadas {
                    if (palpite - numero_secreto) > 50 {
                        println!("{}", "Muito alto! Tente um número bem menor.".red());
                    } else if (palpite - numero_secreto) > 20 {
                        println!("{}", "Um pouco alto! Tente um número menor.".red());
                    } else {
                        println!("{}", "Quase! Um pouco menor.".red());
                    }
                } else {
                    println!("{}", "Muito alto! Perdeu uma vida.".red());
                }
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

        if points <= 0  || (usar_cronometrado && inicio_fase.unwrap().elapsed() > tempo_limite) {
         break;   
        }
    

    }
    if points <= 0 {
        break;
    }

    fase += 1;
    points += 5;
    println!("{}",  format!("Parabéns! Você passou para a fase {} .\n", fase).green())
    
    
    


  }
  println!("Reiniciando o jogo...\n");

}
}
