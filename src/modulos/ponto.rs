// Define uma estrutura chamada `Ponto` com o atributo `Debug` para permitir a exibição
// formatada para debug e `Clone` para permitir a clonagem de instâncias da estrutura.
#[derive(Debug, Clone)]
pub struct Ponto {
    // O campo `id` armazena um identificador único do tipo `usize` para o ponto.
    pub id: usize,
    // O campo `px` armazena a coordenada x do ponto no tipo de dado de ponto flutuante `f64`.
    pub px: f64,
    // O campo `py` armazena a coordenada y do ponto no tipo de dado de ponto flutuante `f64`.
    pub py: f64,
}

// Implementa métodos associados à estrutura `Ponto`.
impl Ponto {
    // Define um método chamado `distancia` que calcula a distância euclidiana entre
    // o ponto atual (`self`) e outro ponto (`outro`). O método retorna um valor do tipo `f64`.
    pub fn distancia(&self, outro: &Ponto) -> f64 {
        // Calcula a diferença entre as coordenadas x dos dois pontos, eleva ao quadrado,
        // depois calcula a diferença entre as coordenadas y dos dois pontos, eleva ao quadrado,
        // soma esses dois valores e então calcula a raiz quadrada da soma.
        ((self.px - outro.px).powi(2) + (self.py - outro.py).powi(2)).sqrt()
    }
}