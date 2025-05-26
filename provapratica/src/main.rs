fn main() {
    // Array fixo pré-definido
    let mut arr = [34, 7, 23, 32, 5, 62, 31, 12, 43, 3];

    // Imprimindo o array original
    println!("Array original: {:?}", arr);

    // Ordenando o array usando a função nativa .sort()
    arr.sort();

    // Imprimindo o array ordenado
    println!("Array ordenado: {:?}", arr);

}

struct VetorOrdenavel {
    dados: [i32; 10],
}

impl VetorOrdenavel {
    // Cria um novo vetor com dados fixos
    fn novo() -> Self {
        VetorOrdenavel {
            dados: [34, 7, 23, 32, 5, 62, 31, 12, 43, 3],
        }
    }

    // Exibe os dados do vetor
    fn exibir(&self) {
        println!("{:?}", self.dados);
    }

    // Método público para ordenar
    fn ordenar(&mut self) {
        Self::quicksort(&mut self.dados);
    }

    // Função interna de quicksort
    fn quicksort(vetor: &mut [i32]) {
        let len = vetor.len();
        if len <= 1 {
            return;
        }

        let pivot_index = len / 2;
        let pivot = vetor[pivot_index];

        let mut left = 0;
        let mut right = len - 1;

        while left <= right {
            while vetor[left] < pivot {
                left += 1;
            }
            while vetor[right] > pivot {
                right = right.saturating_sub(1);
            }
            if left <= right {
                vetor.swap(left, right);
                left += 1;
                right = right.saturating_sub(1);
            }
        }

        if right > 0 {
            Self::quicksort(&mut vetor[0..=right]);
        }
        if left < len {
            Self::quicksort(&mut vetor[left..]);
        }
    }
}

fn main() {
    // Criando o vetor fixo
    let mut vetor = VetorOrdenavel::novo();

    println!("Array original:");
    vetor.exibir();

    // Ordenando com QuickSort
    vetor.ordenar();

    println!("Array ordenado:");
    vetor.exibir();
}
