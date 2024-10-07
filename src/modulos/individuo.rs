// Importa traços (traits) necessários do crate `rand` para gerar números aleatórios
use rand::Rng;

// Importa a estrutura `Ponto` do módulo `ponto` dentro do módulo `modulos`
use crate::modulos::ponto::Ponto;

// Define uma constante que representa o número de pontos
const NUM_PONTOS: usize = 10;

// Define uma estrutura (struct) chamada `Individuo`
// `#[derive(Debug, Clone)]` permite que a estrutura derive as funcionalidades de debug e clonagem, automaticamente
#[derive(Debug, Clone)]
pub struct Individuo {
    pub rota: Vec<usize>, // Vetor que armazena a rota como índices de pontos
    pub custo: f64,       // Custo total da rota
}

// Implementa métodos para a estrutura `Individuo`
impl Individuo {
    // Método para calcular o custo da rota
    // Recebe uma referência imutável para uma matriz de distâncias
    pub fn calcular_custo(&mut self, dist: &[[f64; NUM_PONTOS]]) {
        self.custo = 0.0; // Inicializa o custo como 0.0
        // Percorre a rota do primeiro ponto ao penúltimo
        for i in 0..NUM_PONTOS - 1 {
            // Adiciona a distância entre ponto atual e o próximo ponto ao custo total
            self.custo += dist[self.rota[i]][self.rota[i + 1]];
        }
        // Adiciona a distância entre o último ponto e o primeiro ponto para fechar o ciclo
        self.custo += dist[self.rota[NUM_PONTOS - 1]][self.rota[0]];
    }
}

// Função que gera uma nova rota aleatória
// Recebe uma fatia (slice) de pontos e uma matriz de distâncias
pub fn gerar_rota(_pontos: &[Ponto], dist: &[[f64; NUM_PONTOS]]) -> Individuo {
    let mut rng = rand::thread_rng(); // Cria um gerador de números aleatórios
    // Cria um vetor de índices disponíveis de 0 a NUM_PONTOS-1
    let mut disponiveis: Vec<usize> = (0..NUM_PONTOS).collect();
    let mut rota = Vec::new(); // Inicializa um vetor vazio para a rota

    // Loop para gerar uma rota aleatória, selecionando pontos disponíveis
    for _ in 0..NUM_PONTOS {
        // Gera um índice aleatório dentro do intervalo de pontos disponíveis
        let indice = rng.gen_range(0..disponiveis.len());
        // Adiciona o ponto selecionado à rota
        rota.push(disponiveis[indice]);
        // Remove o ponto selecionado da lista de pontos disponíveis
        disponiveis.swap_remove(indice);
    }

    // Cria um novo Individuo com a rota gerada e custo inicial de 0.0
    let mut individuo = Individuo { rota, custo: 0.0 };
    // Calcula o custo da rota gerada
    individuo.calcular_custo(dist);
    // Retorna o individuo com a rota e custo calculado
    individuo
}

// Função para aplicar mutação em um indivíduo
// Recebe uma referência mutável para um Individuo
pub fn mutacao(ind: &mut Individuo) {
    let mut rng = rand::thread_rng(); // Cria um gerador de números aleatórios
    // Gera dois índices aleatórios distintos dentro do intervalo de pontos da rota
    let ponto1 = rng.gen_range(0..NUM_PONTOS);
    let ponto2 = rng.gen_range(0..NUM_PONTOS);
    // Usa uma técnica de mutação bi para trocar os pontos
    if ponto1 != ponto2 {
        ind.rota.swap(ponto1, ponto2);
    }
}