use std::collections::HashMap;

fn main() {
    //1. Criação do hashmap
    let mut estoque = HashMap::new();

    //1. inserção de valores
    estoque.insert("banana", 100);
    estoque.insert("pepino", 50);
    estoque.insert("maça", 2);
    estoque.insert("caqui", 20);

    //2. Acessar de forma segura os valores do HashMap
    if let Some(qtde) = estoque.get("maça"){
        println!("Temos {} maças em estoque!", qtde);
    }

    //3. Atualizando estoque com entry()
    estoque.entry("pepino").and_modify(|qtde| *qtde += 100);
    if let Some(qtde) = estoque.get("pepino"){
        println!("Temos {} pepinos em estoque!", qtde);
    }
    //4. Remover caqui da lista
    estoque.remove("caqui");
   // println!("{:?}", estoque);

    //5. Filtrar todas as frutas acima de 100 unidades
    estoque.retain(|fruta, &mut qtde| qtde > 100);
    println!("{:?}", estoque);

    //6. Limpeza total!
    estoque.clear();
    println!("{:?}", estoque);


}
