use std::cell::RefCell;
use std::rc::Rc;


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//      References cycles

    // Reference cycles é um dos problemas que podem ser gerados ao se ultilizar
    // Rc com RefCell, e são causados quando dois SmartP possuem
    // multipla referencia mutavel entre sí.
    // Ou seja quando um SmartsP "A" retem dentro de sí a referencia mutavel
    // sobre o SmartP "B" e o SmartsP "B" retem dentro de sí a referencia mutavel
    // sobre o SmartP "A", assim o reference counter não consegue saber quantos possuidores
    // cada um dos SmartP possuem e dessa maneira temos um loop.

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//                  Forçando um Reference cycle

    enum Infinity2 {
        Bank(i32, RefCell<Rc<Infinity2>>),
        Nop
    }

    impl Infinity2 {
        /// Metodo que retorna o segundo valor contido dentro de Infinity2::Bank()
        fn return_value(&self) -> Option<&RefCell<Rc<Infinity2>>> {
            match self {
                Bank(_, data) => Some(data),
                Nop => None
            }
        }
    }

    use Infinity2::{Bank, Nop};


    fn reference_cycle() {
        // RefCell(Rc(5, RefCell(Rc(Nop)))); -> [5, Nop]
        // Elemento que permite multiplas referencias usando "Rc" (Reference counter)
        // e q possui um elemento de multipla referencia mutavel
        let a = Rc::new(Bank(5, RefCell::new(Rc::new(Nop))));

        // Elemento que possui referencia mutavel do elemento "a"
        let b = Rc::new(Bank(5, RefCell::new(Rc::clone(&a))));

        // O evento match com o metodo "a.return_value()"
        // retorna o elemento com multipla referencia mutavel de a 
        match a.return_value() {
            // a função referente "Some(dt)" dentro do match atualiza o valor
            // do "Nop" dentro do elemento "a", 
            // colocando a referencia do elemento "b" no lugar
            Some(dt) => {*dt.borrow_mut() = Rc::clone(&b);},
            None => {},
        }

        // Com os processos que ocorreram acima, o elemento "a" se referencia ao
        // elemento "b", e o elemento "b" se referencia ao elemento "a" assim estourando a pilha
        // e aumentando o reference counter dos elementos "a" e "b" para 2
        println!("'a' reference counters {}", Rc::strong_count(&a));
        println!("'b' reference counters {}", Rc::strong_count(&b));

        // Assim temos um Reference Cycle ou: "Um loop de referencias"
    }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//             Evitando Reference Cycles usando Week

    // Para evitar isso usamos um SmartP feito só para essa situação
    // ele é chamado de Week reference e basicamente ele encapsula o Rc<T> dentro de
    // um Option assim quando nos referimos a ele, não chamamos a estrutura na qual o
    // Rc está se referindo mas mesmo assim ainda temos o SmartP Rc<T> e podemos invoca-lo
    // quando quisermos

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


fn main() {
    reference_cycle();
}