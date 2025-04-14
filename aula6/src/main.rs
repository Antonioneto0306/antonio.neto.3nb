use std::collections::{HashMap, HashSet, VecDeque };

struct Graph {
    adjacency_lyst: HashMap<String, Vec<String>>,

}

impl Graph {
    fn new() -> Self {
        Graph{
            adjacency_lyst: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: &str) {
        self.adjacency_lyst.entry(vertex.to_string()).or_insert(Vec::new());
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.adjacency_lyst.entry(from.to_string()).or_insert(Vec::new()).push(to.to_string());
        self.adjacency_lyst.entry(to.to_string()).or_insert(Vec::new()).push(from.to_string());
    }

    fn dfs(&self, start: &str) {
        let mut visited = Hash::new();
        println!("DFS");
        self.dfs_recursive(start, &mut visited);
    }

    dn dfs_recursive(&self, vertex: &str, visited: &mut HashSet<String>) {
        if visited.contains(vertex) {
            return;
        }
        println!("{}", vertex);
        visited.insert((vertex.to_string()));

        if let Some(neighbors) = sel.adjacency_lyst.get(vertex) {
            for neighbor in neighbors {
                self.dfs_recursive(neighbor, visited);
            }
        }
    }

    fn bfs(&self start: &str) {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string());

        println!("BFS")

        while let Some(current) = queue.pop_front() {
            println!("{}", current);

            if let Some(neighbors) = self.adjacency_lyst.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
    }
}

