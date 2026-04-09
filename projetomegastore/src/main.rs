use std::collections::{HashMap, HashSet, VecDeque};


#[derive(Debug)]
struct Grafo {
    conexoes: HashMap<String, Vec<(String, f32)>>,
}


impl Grafo {
    fn novo() -> Self {
        Grafo {
            conexoes: HashMap::new(),
        }
    }


    fn adicionar_produto(&mut self, produto: &str) {
        self.conexoes
            .entry(produto.to_string())
            .or_insert_with(Vec::new);
    }


    fn conectar(&mut self, p1: &str, p2: &str, peso: f32) {
        self.conexoes
            .entry(p1.to_string())
            .or_insert_with(Vec::new)
            .push((p2.to_string(), peso));


        self.conexoes
            .entry(p2.to_string())
            .or_insert_with(Vec::new)
            .push((p1.to_string(), peso));
    }


    //  Recomendação profissional
    fn recomendar(
        &self,
        produto: &str,
        profundidade_max: usize,
        top_n: usize,
    ) -> Vec<(String, f32)> {
        let mut visitados: HashSet<String> = HashSet::new();
        let mut fila: VecDeque<(String, f32, usize)> = VecDeque::new();
        let mut scores: HashMap<String, f32> = HashMap::new();


        fila.push_back((produto.to_string(), 1.0, 0));


        while let Some((atual, peso_acumulado, profundidade)) = fila.pop_front() {
            if profundidade > profundidade_max {
                continue;
            }


            if !visitados.insert(atual.clone()) {
                continue;
            }


            if let Some(vizinhos) = self.conexoes.get(&atual) {
                for (vizinho, peso) in vizinhos {
                    let score = peso_acumulado * peso;


                    *scores.entry(vizinho.clone()).or_insert(0.0) += score;


                    fila.push_back((vizinho.clone(), score, profundidade + 1));
                }
            }
        }


        scores.remove(produto);


        let mut resultado: Vec<(String, f32)> = scores.into_iter().collect();


        //  Ordena por score
        resultado.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());


        //  Retorna só top N (como sistemas reais)
        resultado.into_iter().take(top_n).collect()
    }
}


fn main() {
    let mut grafo = Grafo::novo();


    let produtos = vec![
        "Notebook",
        "Mouse",
        "Teclado",
        "Monitor",
        "Headset",
        "Webcam",
        "Cadeira Gamer",
    ];


    for p in produtos {
        grafo.adicionar_produto(p);
    }


    grafo.conectar("Notebook", "Mouse", 0.9);
    grafo.conectar("Notebook", "Teclado", 0.8);
    grafo.conectar("Notebook", "Monitor", 0.95);
    grafo.conectar("Mouse", "Headset", 0.7);
    grafo.conectar("Teclado", "Headset", 0.6);
    grafo.conectar("Monitor", "Webcam", 0.75);
    grafo.conectar("Cadeira Gamer", "Headset", 0.5);


    let recomendacoes = grafo.recomendar("Notebook", 2, 5);


    println!("\n Top recomendações para Notebook:\n");


    for (produto, score) in recomendacoes {
        println!("{} → {:.3}", produto, score);
    }
}

