use std::cell::RefCell;

/*fn main() {
    let data = RefCell::new(5);
    {
        let mut borrowed = data.borrow_mut(); // Empréstimo mutável em tempo de execução
        *borrowed *= 3;
    }
    println!("Valor atualizado: {}", data.borrow());
}*/

fn  main(){
    let &str name = RefCell::new(Lucas);
    {

        let mut borrowed = name.borrow_mut();
        borrowed += silva;
    }
}




