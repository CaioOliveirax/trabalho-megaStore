mod tests {
    use super::*;


    fn criar_grafo_teste() -> Grafo {
        let mut g = Grafo::novo();


        let produtos = [
            "Notebook",
            "Mouse",
            "Teclado",
            "Monitor",
            "Headset",
        ];


        for p in produtos {
            g.adicionar_produto(p);
        }


        g.conectar("Notebook", "Mouse", 1.0);
        g.conectar("Notebook", "Teclado", 0.8);
        g.conectar("Mouse", "Headset", 0.5);
        g.conectar("Teclado", "Headset", 0.4);


        g
    }


    #[test]
    fn deve_adicionar_produto() {
        let mut g = Grafo::novo();
        g.adicionar_produto("Notebook");


        assert!(g.conexoes.contains_key("Notebook"));
    }


    #[test]
    fn deve_conectar_produtos() {
        let mut g = Grafo::novo();


        g.adicionar_produto("A");
        g.adicionar_produto("B");
        g.conectar("A", "B", 1.0);


        let conexoes = g.conexoes.get("A").unwrap();
        assert_eq!(conexoes.len(), 1);
        assert_eq!(conexoes[0].0, "B");
    }


    #[test]
    fn deve_retornar_recomendacoes() {
        let g = criar_grafo_teste();


        let rec = g.recomendar("Notebook", 1, 5);


        assert!(!rec.is_empty());
    }


    #[test]
    fn deve_respeitar_top_n() {
        let g = criar_grafo_teste();


        let rec = g.recomendar("Notebook", 2, 1);


        assert_eq!(rec.len(), 1);
    }


    #[test]
    fn nao_deve_incluir_produto_original() {
        let g = criar_grafo_teste();


        let rec = g.recomendar("Notebook", 2, 5);


        let contem = rec.iter().any(|(p, _)| p == "Notebook");
        assert!(!contem);
    }


    #[test]
    fn deve_ordenar_por_score() {
        let g = criar_grafo_teste();


        let rec = g.recomendar("Notebook", 1, 5);


        for i in 1..rec.len() {
            assert!(rec[i - 1].1 >= rec[i].1);
        }
    }


    #[test]
    fn produto_inexistente_retorna_vazio() {
        let g = criar_grafo_teste();


        let rec = g.recomendar("ProdutoInvalido", 2, 5);


        assert!(rec.is_empty());
    }


    #[test]
    fn profundidade_afeta_resultado() {
        let g = criar_grafo_teste();


        let rec1 = g.recomendar("Notebook", 1, 5);
        let rec2 = g.recomendar("Notebook", 2, 5);


        assert!(rec2.len() >= rec1.len());
    }
}
