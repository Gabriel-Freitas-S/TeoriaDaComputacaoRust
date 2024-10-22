use crate::modulos::individuo::{mutacao, Individuo}; // Importa o módulo `individuo` e suas funcionalidades.
use rand::Rng; // Importa a trait `Rng` da biblioteca `rand` para geração de números aleatórios.

pub(crate) const NUM_PONTOS: usize = 10; // Constante que define o número de pontos (tamanho fixo).
pub(crate) const TAM_POPULACAO: usize = 100; // Constante que define o tamanho da população.

/// Função `roleta` que seleciona um índice de um indivíduo na população.
/// Recebe a referência à população e o custo total como parâmetros.
/// Retorna o índice de um indivíduo na população.
pub fn roleta(populacao: &Vec<Individuo>, custo_total: f64) -> usize {
    let mut rng = rand::thread_rng(); // Cria um gerador de números aleatórios.
    let aleatorio = rng.gen_range(0.0..custo_total); // Gera um número aleatório entre 0.0 e `custo_total`.
    let mut soma_custo = 0.0; // Inicializa a soma dos custos.

    // Itera sobre a população acumulando os custos até o valor aleatório.
    for (i, ind) in populacao.iter().enumerate() {
        soma_custo += ind.custo; // Acumula os custos dos indivíduos.
        if soma_custo >= aleatorio {
            // Quando a soma dos custos ultrapassa o valor aleatório...
            return i; // Retorna o índice do indivíduo selecionado.
        }
    }

    0 // Caso não encontre algum indivíduo (tecnicamente nunca acontecerá), retorna 0.
}

/// Função `cruzamento` que realiza a busca local entre dois indivíduos usando Path Relinking.
/// Recebe referências a dois indivíduos (`a` e `b`) e a matriz de distâncias.
/// Retorna um novo indivíduo resultante do path relinking.
pub fn cruzamento(a: &Individuo, b: &Individuo, dist: &[[f64; NUM_PONTOS]]) -> Individuo {
    // Clona o primeiro indivíduo `a` para iniciar com a suposição de que é o melhor.
    let mut melhor = a.clone();
    // Armazena o custo associado ao melhor caminho encontrado até agora.
    let mut melhor_custo = melhor.custo;
    // Clona novamente `a` para iniciar o processo de otimização.
    let mut atual = a.clone();

    // Itera sobre cada gene na rota do indivíduo `a` para compará-la com `b`.
    for i in 0..NUM_PONTOS {
        // Verifica se o gene atual na posição `i` difere entre `atual` e `b`.
        if atual.rota[i] != b.rota[i] {
            // Se os genes diferem, itera para explorar trocas potenciais a partir da posição `i+1`.
            for j in i + 1..NUM_PONTOS {
                // Realiza a troca apenas se o gene na posição `j` de `atual` coincide com o gene na posição `i` de `b`.
                if atual.rota[j] == b.rota[i] {
                    // Troca os genes nas posições `i` e `j` em `atual`.
                    atual.rota.swap(i, j);
                    // Recalcula o custo do percurso após a troca.
                    atual.calcular_custo(dist);

                    // Se o novo custo é menor, atualiza o melhor percurso conhecido.
                    if atual.custo < melhor_custo {
                        melhor_custo = atual.custo;
                        melhor = atual.clone();
                    }

                    // Reverte a troca realizada para tentar outras combinações.
                    atual.rota.swap(i, j);
                }
            }
        }
    }

    // Retorna o indivíduo que representa o melhor caminho encontrado após avaliar trocas possíveis.
    melhor
}
/// Função `novas_geracoes` que gera novas gerações de indivíduos.
/// Recebe uma referência mutável à população e a matriz de distâncias.
pub fn novas_geracoes(populacao: &mut Vec<Individuo>, dist: &[[f64; NUM_PONTOS]]) {
    // Loop para gerar 100 novas gerações
    for _ in 1..=100 {
        // Calcula o custo total da população atual
        let total_custo: f64 = populacao.iter().map(|ind| ind.custo).sum();

        // Cria um novo vetor para armazenar a nova geração
        let mut nova_populacao = Vec::with_capacity(TAM_POPULACAO);

        // Loop para gerar 40 novos indivíduos
        for _ in 0..40 {
            // Seleciona dois pais aleatoriamente usando o método da roleta
            let a_idx = roleta(populacao, total_custo);
            let b_idx = roleta(populacao, total_custo);

            // Obtém os pais da população atual
            let a = &populacao[a_idx];
            let b = &populacao[b_idx];

            // Gera um novo indivíduo a partir do cruzamento dos pais
            let mut filho = cruzamento(a, b, dist);

            // Aplica mutação ao novo indivíduo
            mutacao(&mut filho);

            // Calcula o custo do novo indivíduo
            filho.calcular_custo(dist);

            // Adiciona o novo indivíduo à nova população
            nova_populacao.push(filho);
        }

        // Remove os 40 piores indivíduos da população atual
        populacao.truncate(60);

        // Adiciona os 40 novos indivíduos à população atual
        populacao.append(&mut nova_populacao);

        // Ordena a população atual pelo custo, do menor para o maior
        populacao.sort_by(|a, b| a.custo.partial_cmp(&b.custo).unwrap_or(std::cmp::Ordering::Equal)); 

    }
}
