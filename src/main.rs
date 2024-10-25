use std::io; // Importa o módulo de entrada e saída

fn main() {
   
    println!("Digite uma frase:");

    // Aqui estou criando uma String vazia
    let mut input = String::new();
  
    // Lendo o input 
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a linha");

    // Chama a função para contar as palavras
    let word_count = count_words(&input);

    
    println!("Número de palavras: {}", word_count);
}

// Função que conta o número de palavras em uma string
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}
