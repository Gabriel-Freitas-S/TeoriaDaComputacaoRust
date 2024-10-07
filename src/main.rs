// Declaração do módulo raiz 'modulos' que contém submódulos.
mod modulos;

// Importação do módulo 'rand' para gerar números aleatórios.
use rand::Rng;

// Importação do módulo 'Ponto' do módulo 'ponto'.
use modulos::ponto::Ponto;

// Importação dos itens 'Individuo' e 'gerar_rota' do módulo 'individuo'.
use modulos::individuo::{gerar_rota, Individuo};

// Importação do itens 'mostrar_populacao' e 'clear_terminal'  do módulo 'utils'.
use modulos::utils::{clear_terminal, mostrar_populacao};

// Importação dos itens 'novas_geracoes', 'NUM_PONTOS' e 'TAM_POPULACAO' do módulo 'genetica'.
use modulos::genetica::{novas_geracoes, NUM_PONTOS, TAM_POPULACAO};

// Função principal que será executada quando o programa iniciar.
fn main() {
    // Criação de um gerador de números aleatórios.
    let mut rng = rand::thread_rng();

    // Criação de um vetor de 'Ponto' com capacidade para `NUM_PONTOS` itens.
    let mut pontos: Vec<Ponto> = Vec::with_capacity(NUM_PONTOS);

    clear_terminal(); // Limpa a tela do terminal.

    // Loop para preencher o vetor de pontos com coordenadas aleatórias.
    for i in 0..NUM_PONTOS {
        pontos.push(Ponto {
            id: i,
            px: rng.gen_range(0.0..=360.0),
            py: rng.gen_range(0.0..=360.0),
        });
    }

    // Impressão dos pontos gerados.
    println!("Pontos:");
    println!("| {:>4} | {:>7} | {:>7} |", "Id", "PX", "PY");
    println!("|------|---------|---------|");

    // Itera sobre cada ponto no vetor e imprime suas coordenadas.
    for ponto in &pontos {
        println!(
            "| {:>4} | {:>7.2} | {:>7.2} |",
            ponto.id, ponto.px, ponto.py
        );
    }

    // Criação de uma matriz para armazenar as distâncias entre cada par de pontos.
    let mut dist = [[0.0; NUM_PONTOS]; NUM_PONTOS];

    // Loop duplo para calcular as distâncias euclidianas entre todos os pontos.
    for i in 0..NUM_PONTOS {
        for j in 0..NUM_PONTOS {
            dist[i][j] = pontos[i].distancia(&pontos[j]);
        }
    }

    // Impressão da matriz de distâncias euclidianas.
    println!("\nMatriz de Distâncias Euclidianas:");
    print!("|      |");

    // Primeira linha da matriz com os IDs dos pontos.
    for ponto in &pontos {
        print!(" {:>7} |", ponto.id);
    }
    println!();

    // Linhas subsequentes da matriz com as distâncias.
    for i in 0..NUM_PONTOS {
        print!("| {:>4} |", pontos[i].id);
        for j in 0..NUM_PONTOS {
            print!(" {:>7.2} |", dist[i][j]);
        }
        println!();
    }

    // Criação de um vetor para armazenar a população de 'Individuo' (rotas).
    let mut populacao: Vec<Individuo> = Vec::with_capacity(TAM_POPULACAO);

    // Geração inicial das rotas para a população.
    for _ in 0..TAM_POPULACAO {
        populacao.push(gerar_rota(&pontos, &dist));
    }

    // Ordenação da população pelo custo da rota em ordem crescente.
    populacao.sort_by(|a, b| {
        a.custo
            .partial_cmp(&b.custo)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Impressão da população ordenada pelo custo da rota.
    println!("\nPopulação Ordenada pelo Custo da Rota:");
    mostrar_populacao(&populacao);

    // Evolução da população através de novas gerações.
    novas_geracoes(&mut populacao, &dist);

    // Impressão da população final após a evolução.
    println!("\nPopulação Final Ordenada pelo Custo da Rota:");
    mostrar_populacao(&populacao);
}
