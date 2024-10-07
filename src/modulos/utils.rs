// Aqui estamos importando a estrutura `Individuo` do módulo `individuo`, que está no módulo `modulos`.
// Presumimos que exista um arquivo `individuo.rs` dentro de um diretório chamado `modulos`.
use crate::modulos::individuo::Individuo;
use std::process::Command;

// Define a função `clear_terminal`, que não recebe parâmetros e não retorna nenhum valor.
pub fn clear_terminal() {
    // A primeira instrução verifica a configuração do sistema operacional de destino
    if cfg!(target_os = "windows") {
        // Se o sistema operacional de destino for Windows, será executado o comando "cls" via "cmd".
        // Command::new cria um novo processo para executar o comando "cmd".
        // `.args(&["/C", "cls"])` define os argumentos a serem passados para o "cmd", onde "/C" diz para o "cmd" executar o comando especificado e "cls" é o comando que limpa o terminal.
        // `.status().unwrap()` executa o comando e espera pelo resultado. O `unwrap()` é utilizado para tratar qualquer erro que possa ocorrer; nesse caso, ele apenas interrompe a execução se um erro ocorrer.
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        // Se o sistema operacional não for Windows, assume-se que é um sistema similar a Unix (como Linux ou macOS).
        // Command::new cria um novo processo para executar o comando "clear", que é usado para limpar o terminal no Unix.
        // `.status().unwrap()` aqui também executa o comando e trata possíveis erros de execução da mesma maneira.
        Command::new("clear").status().unwrap();
    }
}

// Definição da função `mostrar_populacao`, que recebe uma fatia (slice) imutável de `Individuo`.
// A fatia é uma referência a uma porção contínua de um array, vetor ou outra coleção.
pub fn mostrar_populacao(populacao: &[Individuo]) {
    // Iniciamos um loop sobre a fatia `populacao`, utilizando o método `iter()` para obter um iterador.
    // `enumerate()` adiciona índices ao iterador.
    for (i, individuo) in populacao.iter().enumerate(){
        // `print!` é utilizado para imprimir na tela sem adicionar uma nova linha ao final.
        // Aqui imprimimos o índice do indivíduo(1 ate 100) e iniciamos a definição da rota.
        print!("Indivíduo {}: Rota = [ ", i+1);

        // Iteramos sobre cada ponto na rota do indivíduo. `&individuo.rota` é uma referência imutável ao vetor `rota`.
        // Usamos o `&` para desreferenciar `ponto`, obtendo o valor contido.
        for &ponto in &individuo.rota {
            // `print!` é usado novamente para imprimir cada ponto da rota.
            print!("{} ", ponto);
        }

        // `println!` é utilizado para imprimir uma linha com um caractere de nova linha ao final.
        // Aqui concluímos a impressão da rota e imprimimos o custo do indivíduo com duas casas decimais.
        println!("] Custo = {:.2}", individuo.custo);
    }
}