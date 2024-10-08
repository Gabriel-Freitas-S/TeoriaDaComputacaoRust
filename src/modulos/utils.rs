// Aqui estamos importando a estrutura `Individuo` do módulo `individuo`, que está no módulo `modulos`.
use crate::modulos::individuo::Individuo;
use std::process::Command;
use super::{super::NUM_PONTOS,super::Ponto};
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

// Esta função `mostrar_populacao` recebe um slice imutável de `Individuo` como parâmetro,
// que é uma referência a uma parte contínua de uma coleção, como um array ou vetor,
// contendo dados de múltiplos indivíduos. A função exibe essas informações na tela.
pub fn mostrar_populacao(populacao: &[Individuo]) {
    // Definem-se os códigos ANSI para cores. Esses códigos são sequências de escape
    // que alteram a cor do texto no terminal. Por exemplo, "\x1b[31m" muda a cor do texto para vermelho.
    let red = "\x1b[31m";
    let green = "\x1b[32m";
    let blue = "\x1b[34m";
    let reset = "\x1b[0m";  // Reseta a cor para o padrão do terminal

    // Itera sobre cada indivíduo na população usando um laço `for`,
    // com `enumerate` para obter tanto o índice (posição) quanto o próprio `individuo`.
    for (i, individuo) in populacao.iter().enumerate() {
        // Imprime as informações de cada indivíduo. Usa-se `print!` para não adicionar
        // uma nova linha após a execução, permitindo que o texto continúe na mesma linha.
        print!("{}Indivíduo {}: {} Rota = [ ", green, i+1, red);

        // Itera sobre cada `ponto` na rota do indivíduo, imprimindo-o com a cor vermelha.
        for &ponto in &individuo.rota {
            print!("{}{} ", red, ponto);
        }
        // Após listar todos os pontos da rota, imprime o custo desta com formatação.
        // `println!` é usado para adicionar uma nova linha após a execução.
        println!("] {} Custo = {:.2}{}", blue, individuo.custo, reset);
    }
}

// A função `mostrar_pontos` recebe um slice imutável de `Ponto`, representando uma coleção de pontos,
// e exibe suas coordenadas na tela em formato tabular.
pub fn mostrar_pontos(pontos: &[Ponto]) {
    // Define-se códigos ANSI para cores semelhantes ao da outra função,
    // para alterar cores de texto ao imprimir.
    let yellow = "\x1b[33m";
    let blue = "\x1b[34m";
    let red = "\x1b[31m";
    let reset = "\x1b[0m";

    // Imprime um cabeçalho, seguido de uma nova linha, utilizando `println!`. Coloca "Pontos:" entre
    // dois contextos de cores, começando novamente com `reset` para iniciar com a cor padrão.
    println!("{}Pontos:{}\n", reset, yellow);

    // Imprime o cabeçalho da tabela com os títulos "Id", "PX" e "PY",
    // cada um em sua respectiva cor.
    println!("| {:>4} {}| {:>7} |{} {:>7} |{}", "Id",blue, "PX",red ,"PY",yellow);
    println!("|------{}|---------|{}---------|{}",blue,red,yellow);

    // Itera por cada ponto na coleção recebida, exibindo suas informações em formato tabular.
    for ponto in pontos {
        println!("| {:>4} {}| {:>7.2} |{} {:>7.2} |{}", ponto.id,blue ,ponto.px,red, ponto.py,yellow);
    }
    // Após a iteração e impressão de todos os pontos, faz uma quebra de linha para finalizar a tabela.
    println!();
}

// A função `mostrar_matriz_distancias` recebe dois argumentos: `pontos`, um slice imutável de `Ponto`,
// e `dist`, uma matriz bidimensional de distâncias calculadas entre os pontos.
// Esta função exibe a matriz de distâncias euclidianas de cada ponto para os demais na tela.
pub fn mostrar_matriz_distancias(pontos: &[Ponto], dist: &[[f64; NUM_PONTOS]]) {
    // Define códigos ANSI para cores, como cyan, verde e amarelo.
    let cyan = "\x1b[36m";
    let green = "\x1b[32m";
    let yellow = "\x1b[33m";
    let reset = "\x1b[0m";

    // Imprime títulos iniciais, com cores definidas ao texto "Matriz de Distâncias Euclidianas".
    println!("{}Matriz de Distâncias Euclidianas:{}\n",reset,cyan);

    // Inicia a impressão da linha do cabeçalho da matriz, começando com "|      |"
    print!("|      |");

    // Itera pelos pontos e imprime uma coluna para cada um, com suas identidades (ID),
    // precedida e seguida pela cor ciano.
    for ponto in pontos {
        print!(" {}{:>7}{} |", green, ponto.id,cyan);
    }
    // Quebra a linha após a lista de IDs.
    println!();

    // Itera sobre o índice de cada ponto `i`, necessário para acessar o array de distâncias.
    for i in 0..NUM_PONTOS {
        // Imprime o ID do ponto atual (linha) no início da linha da matriz com cor.
        print!("|{} {:>4} {}|", green, pontos[i].id, cyan);

        // Itera sobre cada possível ponto de destino `j` imprimindo a distância de `i` para `j`.
        for j in 0..NUM_PONTOS {
            print!("{} {:>7.2} {}|", yellow, dist[i][j], cyan);
        }
        // Quebra a linha ao final de cada linha de distâncias.
        println!();
    }
    // Reseta a cor ao padrão após finalizar a impressão da matriz.
    println!("{}",reset);
}