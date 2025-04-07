// Testes Simples para Implementação de Árvore Binária de Busca (BST)
// Os alunos devem implementar a árvore para fazer estes testes passarem


#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    // Criar uma nova árvore vazia
    fn new() -> Self {
        BST { root: None }
    }

    // Verificar se a árvore está vazia
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // Inserir um valor na árvore
    fn insert(&mut self, value: i32) {
        let mut current = &mut self.root;
        loop {
            match current {
                None => {
                    *current = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }));
                    return;
                }
                Some(node) => {
                    if value < node.value {
                        current = &mut node.left;
                    } else {
                        current = &mut node.right;
                    }
                }
            }
        }
    }

    // Buscar um valor na árvore
    fn search(&self, value: i32) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            if value == node.value {
                return true;
            } else if value < node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        
        false
    }
}

#[cfg(test)]
mod bst_tests {
    use super::*;

    #[test]
    fn test_bst_new_and_empty() {
        let bst = BST::new();
        assert!(bst.is_empty());
    }

    #[test]
    fn test_bst_insert_and_search() {
        let mut bst = BST::new();
        
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        
        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        assert!(!bst.search(20));
        assert!(!bst.is_empty());
    }
}
