use petgraph::dot::{Dot, Config};
use petgraph::graph::Graph;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut graph = Graph::<&str, &str>::new();

    //Adiciona Nós
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    
    //Arestas
    graph.add_edge(a, e, "");
    graph.add_edge(d, b, "");
    graph.add_edge(c, a, "");
    graph.add_edge(e, b, "");
    graph.add_edge(c, d, "");
    
    //.dot
    let dot_output = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let mut file = File::create("graph.dot")?;
    file.write_all(dot_output.as_bytes())?;

    println!("Arquivo gravado com sucesso");
    Ok(())

}