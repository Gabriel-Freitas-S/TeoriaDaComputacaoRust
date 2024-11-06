# Algoritmo Genético para o Problema do Caixeiro Viajante (TSP)

Este projeto implementa um algoritmo genético para resolver o Problema do Caixeiro Viajante (TSP). O objetivo é encontrar a rota mais curta que passa por todos os pontos uma única vez e retorna ao ponto de partida.

## Estrutura de Arquivos do Projeto

*   `src/main.rs`: O ponto de entrada do código. Inicializa a população, calcula as distâncias entre os pontos e executa o algoritmo genético.
    
    *   `main`: Função que orquestra a execução geral incluindo geração de pontos, cálculo de distâncias, e início das iterações do algoritmo genético.
        
        ```rust
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
        
            // ... (código para calcular distâncias e executar o algoritmo genético)
        }
        ```
        
*   `src/modulos/mod.rs`: Define os módulos do projeto.
    
    *   Modulariza componentes como `genetica`, `individuo`, `ponto` e `utils`.
        
        ```rust
        // O módulo `genetica` é importado.
        pub(crate) mod genetica;
        
        // O módulo `individuo` é importado.
        pub(crate) mod individuo;
        
        // O módulo `ponto` é importado.
        pub(crate) mod ponto;
        
        // O módulo `utils` é importado.
        pub(crate) mod utils;
        ```
        
*   `src/modulos/genetica.rs`: Lida com operações genéticas para criação e evolução de populações.
    
    *   `roleta`: Seleciona um indivíduo baseado no custo.
        
        ```rust
        pub fn roleta(populacao: &Vec<Individuo>, custo_total: f64) -> usize {
            // ... (código para selecionar um indivíduo)
        }
        ```
        
    *   `cruzamento`: Executa o método de Path Relinking entre dois indivíduos.
        
        ```rust
        pub fn cruzamento(a: &Individuo, b: &Individuo, dist: &[[f64; NUM_PONTOS]]) -> Individuo {
            // ... (código para realizar o cruzamento)
        }
        ```
        
    *   `novas_geracoes`: Gera novas gerações de indivíduos a partir das anteriores.
        
        ```rust
        pub fn novas_geracoes(populacao: &mut Vec<Individuo>, dist: &[[f64; NUM_PONTOS]]) {
            // ... (código para gerar novas gerações)
        }
        ```
        
*   `src/modulos/individuo.rs`: Define a estrutura `Individuo` e suas operações.
    
    *   `Individuo`: Estrutura que representa uma solução possível com rota e custo.
        
        ```rust
        #[derive(Debug, Clone)]
        pub struct Individuo {
            pub rota: Vec<usize>, // Vetor que armazena a rota como índices de pontos
            pub custo: f64,       // Custo total da rota
        }
        ```
        
    *   `calcular_custo`: Calcula o custo total de uma rota.
        
        ```rust
        pub fn calcular_custo(&mut self, dist: &[[f64; NUM_PONTOS]]) {
            // ... (código para calcular o custo)
        }
        ```
        
    *   `gerar_rota`: Cria uma nova rota aleatória.
        
        ```rust
        pub fn gerar_rota(_pontos: &[Ponto], dist: &[[f64; NUM_PONTOS]]) -> Individuo {
            // ... (código para gerar uma rota aleatória)
        }
        ```
        
    *   `mutacao`: Troca elementos aleatoriamente na rota para simular mutação.
        
        ```rust
        pub fn mutacao(ind: &mut Individuo) {
            // ... (código para realizar a mutação)
        }
        ```
        
*   `src/modulos/ponto.rs`: Define a estrutura `Ponto` com coordenadas.
    
    *   `Ponto`: Representa um ponto no plano com id e coordenadas x e y.
        
        ```rust
        #[derive(Debug, Clone)]
        pub struct Ponto {
            pub id: usize,
            pub px: f64,
            pub py: f64,
        }
        ```
        
    *   `distancia`: Calcula a distância euclidiana entre dois pontos.
        
        ```rust
        impl Ponto {
            pub fn distancia(&self, outro: &Ponto) -> f64 {
                // ... (código para calcular a distância)
            }
        }
        ```
        
*   `src/modulos/utils.rs`: Contém funções utilitárias para operações gerais.
    
    *   `clear_terminal`: Limpa o terminal.
        
        ```rust
        pub fn clear_terminal() {
            // ... (código para limpar o terminal)
        }
        ```
        
    *   `mostrar_populacao`: Exibe uma lista de indivíduos com suas rotas e custos.
        
        ```rust
        pub fn mostrar_populacao(populacao: &[Individuo]) {
            // ... (código para exibir a população)
        }
        ```
        
    *   `mostrar_pontos`: Exibe as coordenadas dos pontos.
        
        ```rust
        pub fn mostrar_pontos(pontos: &[Ponto]) {
            // ... (código para exibir os pontos)
        }
        ```
        
    *   `mostrar_matriz_distancias`: Exibe a matriz de distâncias entre os pontos.
        
        ```rust
        pub fn mostrar_matriz_distancias(pontos: &[Ponto], dist: &[[f64; NUM_PONTOS]]) {
            // ... (código para exibir a matriz de distâncias)
        }
        ```

## Funcionamento do Algoritmo

**Inicialização:**

*   Geração de pontos com coordenadas aleatórias e cálculo da matriz de distâncias.
    
*   Criação da população inicial com rotas aleatórias e cálculo do custo de cada uma.
    

**Execução do Algoritmo Genético:**

*   Geração iterativa de novas gerações usando operadores de cruzamento e mutação.
    
*   Seleção de indivíduos baseados no custo usando o método de roleta.
    

**Finalização:**

*   Exibição da população final ordenada pelo custo da rota.
    

## Detalhes Técnicos

*   O projeto usa o crate `rand` para geração de números aleatórios.
    
*   A implementação divide tarefas específicas em módulos para organização e manutenção do código.
