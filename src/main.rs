// Declaração do módulo raiz 'modulos' que contém submódulos.
mod modulos;

// Importação do módulo 'rand' para gerar números aleatórios.
use rand::Rng;

// Importação do módulo 'Ponto' do módulo 'ponto'.
use modulos::ponto::Ponto;

// Importação dos itens 'Individuo' e 'gerar_rota' do módulo 'individuo'.
use modulos::individuo::{gerar_rota, Individuo};

// Importação do itens 'mostrar_populacao' e 'clear_terminal'  do módulo 'utils'.
use modulos::utils::{clear_terminal, mostrar_populacao,mostrar_matriz_distancias,mostrar_pontos};

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
            id: i, // Atribuição de um identificador único ao ponto.
            px: rng.gen_range(0.0..=360.0), // Geração de uma coordenada x aleatória.
            py: rng.gen_range(0.0..=360.0), // Geração de uma coordenada y aleatória.
        });
    }

    // Mostra os pontos gerados na tela.
    mostrar_pontos(&pontos);
    
    // Criação de uma matriz para armazenar as distâncias entre cada par de pontos.
    let mut dist = [[0.0; NUM_PONTOS]; NUM_PONTOS];

    // Loop duplo para calcular as distâncias euclidianas entre todos os pontos.
    for i in 0..NUM_PONTOS {
        for j in 0..NUM_PONTOS {
            dist[i][j] = pontos[i].distancia(&pontos[j]); // Calcula a distância entre dois pontos.
        }
    }

    // Criação de um vetor para armazenar a população de 'Individuo' (rotas).
    let mut populacao: Vec<Individuo> = Vec::with_capacity(TAM_POPULACAO);

    // Mostra a matriz de distâncias calculadas.
    mostrar_matriz_distancias(&pontos, &dist);
    
    // Geração inicial das rotas para a população.
    for _ in 0..TAM_POPULACAO {
        populacao.push(gerar_rota(&pontos, &dist)); // Gera uma rota e adiciona à população.
    }

    // Ordenação da população pelo custo da rota em ordem crescente.
    populacao.sort_by(|a, b| {
        a.custo
            .partial_cmp(&b.custo)
            .unwrap_or(std::cmp::Ordering::Equal) // Compara o custo de duas rotas para ordenação.
    });

    // Impressão da população ordenada pelo custo da rota.
    println!("\nPopulação Ordenada pelo Custo da Rota:\n");
    mostrar_populacao(&populacao); // Exibe a população ordenada.

    // Evolução da população através de novas gerações.
    novas_geracoes(&mut populacao, &dist); // Aplica evolução genética à população.

    // Impressão da população final após a evolução.
    println!("\nPopulação Final Ordenada pelo Custo da Rota:\n");
    mostrar_populacao(&populacao); // Exibe a população após a evolução.
}
