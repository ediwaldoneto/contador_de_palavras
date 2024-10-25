# Contador de Palavras em Rust

Este projeto é um simples contador de palavras desenvolvido em Rust. O programa lê uma frase digitada pelo usuário e retorna o número total de palavras presentes na frase.

## Requisitos

- [Rust](https://www.rust-lang.org/) (certifique-se de que o Rust está instalado em seu sistema)

## Como Compilar e Executar

1. Clone o repositório ou baixe o código-fonte.
2. Navegue até o diretório onde o arquivo `contador_palavras.rs` está salvo.
3. Compile o programa usando o comando:

   ```bash
   rustc contador_palavras.rs


## Como Funciona
O programa utiliza a biblioteca padrão de entrada e saída (std::io) para ler uma linha de texto do usuário. Ele divide a string em palavras usando o método split_whitespace() e conta quantas palavras foram encontradas.

Exemplo de Uso

```
Digite uma frase:
Olá, este é um exemplo de frase.
Número de palavras: 7
```


