# trabalho-megaStore

#  Sistema de Recomendação de Produtos em Rust


## 📌 Descrição


Este projeto implementa um **sistema de recomendação de produtos** utilizando **grafos ponderados** em Rust.


A ideia principal é simular como sistemas reais (como e-commerces) sugerem produtos relacionados com base em conexões e relevância.


Cada produto é um nó do grafo, e as conexões entre eles possuem **pesos**, indicando o nível de relação.


---


## 🚀 Como executar o projeto


### ✅ Pré-requisitos


* Rust instalado → https://www.rust-lang.org


### ▶️ Passo a passo


1. Crie um projeto Rust:


```bash
cargo new recomendador
```


2. Substitua o conteúdo do `main.rs` pelo código fornecido.


3. Execute o projeto:


```bash
cargo run
```


---


## 🧪 Exemplo de saída


Ao executar o sistema, você verá algo como:


```bash
Top recomendações para Notebook:


Monitor → 0.950
Mouse → 0.900
Teclado → 0.800
Webcam → 0.712
Headset → 0.630
```


📌 Isso mostra os produtos mais relevantes com base nas conexões do grafo.


---


## 💡 Como funciona


O sistema utiliza uma abordagem baseada em:


### 🔗 Grafo Ponderado


* Cada produto é um **nó**
* Cada conexão tem um **peso (f32)** representando relevância


### 🔍 Algoritmo de Recomendação


A função `recomendar()` usa:


* **Busca em largura (BFS)** com `VecDeque`
* Controle de visitados com `HashSet`
* Cálculo de score acumulado multiplicando pesos
* Limite de profundidade (simula proximidade no grafo)


---


## 🏗️ Estrutura do Código


### 📦 `Grafo`


Responsável por armazenar e manipular os dados


* `novo()` → cria o grafo
* `adicionar_produto()` → adiciona nós
* `conectar()` → cria conexões bidirecionais
* `recomendar()` → gera recomendações inteligentes


---


## ⚙️ Estruturas de Dados Utilizadas


| Estrutura  | Função                            |
| ---------- | --------------------------------- |
| `HashMap`  | Armazenar conexões entre produtos |
| `Vec`      | Lista de vizinhos                 |
| `HashSet`  | Controle de visitados             |
| `VecDeque` | Fila para BFS                     |


---


## 📈 Estratégia de Recomendação


* Começa pelo produto inicial
* Percorre o grafo até uma profundidade máxima
* Multiplica os pesos ao longo do caminho
* Soma os scores de diferentes caminhos
* Ordena e retorna os **Top N produtos**


---


## 🎯 Parâmetros importantes


```rust
recomendar(produto, profundidade_max, top_n)
```


* `produto` → produto base
* `profundidade_max` → até onde explorar o grafo
* `top_n` → quantidade de recomendações retornadas


---


## 🚀 Possíveis melhorias


Este projeto já está bem sólido, mas pode evoluir ainda mais:


*  Persistência em banco de dados
*  Interface gráfica ou API REST
*  Uso de Machine Learning para pesos dinâmicos
*  Integração com sistemas reais de e-commerce
*  Paralelismo para grandes volumes de dados


---


##  Contribuições


Sinta-se à vontade para melhorar o projeto:


```bash
git fork
git checkout -b minha-feature
git commit -m "Nova funcionalidade"
git push origin minha-feature
```


Depois é só abrir um Pull Request 🚀


---


## &#x20;


#  Sistema de Recomendação de Produtos com Grafos em Rust


##  Descrição


Este projeto implementa um **sistema de recomendação de produtos** utilizando **grafos ponderados** em Rust.


A ideia principal é simular como grandes plataformas (como e-commerces) sugerem produtos com base em relações entre itens, utilizando conexões com pesos que representam relevância.


---


##  Funcionalidades


* 📦 Cadastro de produtos
* 🔗 Conexão entre produtos com pesos (relevância)
* 🧠 Algoritmo de recomendação baseado em grafos
* 📊 Ranking dos melhores produtos recomendados
* ⚡ Alta performance com estruturas eficientes


---


## 🛠️ Tecnologias utilizadas


* Rust
* Estruturas de dados:


  * `HashMap`
  * `HashSet`
  * `VecDeque`
* Algoritmo de busca em largura (**BFS adaptado com pesos**)


---


## ▶️ Como executar o projeto


### Pré-requisitos


* Ter o Rust instalado:


```
rustc --version


```


### Passo a passo


1. Clone o repositório:


```
git clone <seu-repositorio>


```


2. Acesse a pasta:


```
cd nome-do-projeto


```


3. Execute o projeto:


```
cargo run


```


---


## 💡 Exemplo de execução


Saída esperada:


```
Top recomendações para Notebook:


Monitor → 0.950
Mouse → 0.900
Teclado → 0.800
Webcam → 0.712
Headset → 0.630


```


---


## 🧠 Como funciona o algoritmo


O sistema utiliza uma abordagem baseada em grafos:


* Cada produto é um **nó**
* Cada conexão representa uma **relação com peso**
* O algoritmo percorre o grafo usando uma **fila (BFS)**


### Estratégia:


1. Começa pelo produto base
2. Percorre os vizinhos até uma profundidade máxima
3. Multiplica os pesos ao longo do caminho
4. Soma os scores para cada produto
5. Ordena os resultados do maior para o menor


---


## 🏗️ Estrutura do sistema


```
src/
 └── main.rs


```


### Componentes principais:


* `Grafo`


  * Armazena conexões entre produtos
* `adicionar_produto`


  * Adiciona um novo nó ao grafo
* `conectar`


  * Cria ligação entre produtos com peso
* `recomendar`


  * Gera recomendações com base no grafo


---


## ⚙️ Estruturas de dados utilizadas


| EstruturaFunção |                                 |
| --------------- | ------------------------------- |
| HashMap         | Armazenar conexões dos produtos |
| HashSet         | Evitar visitas repetidas        |
| VecDeque        | Implementar fila (BFS)          |


---


## 📈 Desempenho e Escalabilidade


* 🔥 Busca eficiente (próxima de O(1) em acessos)
* 📊 Escalável para grandes volumes de dados
* ⚡ Processamento rápido mesmo com múltiplas conexões
* 🧠 Ideal para sistemas de recomendação reais


---


## 🧪 Testes


Para executar testes (caso implemente futuramente):


```
cargo test


```


---


## 🔮 Melhorias futuras


* Implementar interface gráfica ou API
* Persistência de dados (banco de dados)
* Ajuste dinâmico de pesos
* Machine Learning para recomendações mais inteligentes


---


## 🤝 Contribuições


Sinta-se livre para contribuir:


1. Fork o projeto
2. Crie uma branch:


```
git checkout -b minha-feature


```


3. Commit:


```
git commit -m "Nova feature"


```


4. Push:


```
git push origin minha-feature


```


5. Abra um Pull Request 🚀


---


## 📄 Licença


Este projeto está sob a licença MIT.


---


## 👨‍💻 Autor


**Caio Oliveira**
📧 [caitobreme@gmail.com](mailto:caitobreme@gmail.com)
🔗 LinkedIn: https://www.linkedin.com/in/caio-oliveira-78a656359


---


⭐ Se esse projeto te ajudou, considere dar um star!


 


---


##  Autor


**Caio Oliveira**
📍 Taboão da Serra - SP
📧 [caitobreme@gmail.com](mailto:caitobreme@gmail.com)
🔗 LinkedIn: https://www.linkedin.com/in/caio-oliveira-78a656359


---


##  Conclusão


Esse projeto demonstra, de forma prática, como estruturas de dados e algoritmos podem ser usados para construir **sistemas inteligentes de recomendação**.
