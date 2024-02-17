// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//        Introducao Rc<T>

    // Todo elemento do heap qnd é passado para outra variavel, tem seu valor movido
    // ou seja um elemento do heap de maneira comum não
    // podem conter multiplos possuidores.
    // Sabendo-se disso, qnd queremos que mais de um um elemento possa usar um valor
    // no heap usamos os ponteiros padrões "referencias" que copiam o valor
    // desse compartimento de maneira estatica
    // (para rever esse assunto volta para o capitulo 3)

    // Como já comentado, podemos usar SmartP para que um compartimento do heap 
    // possa ter multiplas posses por diversos elementos simultaneamente
    // Para isso usamos o SmartP: Rc<T> (Reference Counter)
    // O Rc<T> armazena dentro de si não apenas o ponteiro referente ao compartimento do heap,
    // mas também a quantidade de elementos que o possuem (reference counter),
    // desta maneira qnd o reference counter 
    // (contador de multiplas referencias possuidoras) for igual a 0,
    // o compartimento heap em questão é desalocado da memoria

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//          Importando Rc e criando ambiente de exemplo

    // Iremos criar uma SmartP que possa ter mais de uma referencia simultaneamente, 
    // ou seja iremps importar e criar o SmartP "Rc"
    use std::rc::Rc;

    // Criando elemento de exemplo que ira ser referida por 2 elementos simultaneamente
    enum List {
        Const(i32, Rc<List>),
        Nil
    }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//              Usando Rc<T> e multipla referencia

    fn usando_rc() {
        // Para criar um SmartP "Rc" iremos usar o metodo Rc::new();
        let a = Rc::new(List::Const(5, Rc::new(List::Const(10, Rc::new(List::Nil)))));

        // O metodo do SmartP Rc<T>: "clone(&Rc<T>);", além de retornar a referencia do elemento "T",
        // serve também para alertar ao Rc inicial e todos os Rc incluidos dentro de sua estrutura
        // (como no caso do elemento "a" que possui varios Rc dentrode sí),
        // que ele deve aumentar o valor de seu contador de multiplas referencias.

        // Não confunda o metodo Rc::clone(&Rc<T>); com o metodo T.clone();
        // O Rc::clone(&Rc<T>); apenas retorna a referencia do valor T e aumenta o contador de multiplas referencias
        // Já o T.clone(); clona de maneira 100% completa o valor de T e o retorna (Lento demais) 

        let b = List::Const(5, Rc::clone(&a));
        let c = List::Const(20, Rc::clone(&a));

        // Podemos também acessar o valor do contador de multiplas referencias
        // para isso usamos Rc::strong_count(&Rc<T>);
        println!("Valor do contador de multiplas referencias do Rc<T> 'a': {}", Rc::strong_count(&a));
    }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//         Limitacao de multipla referencia mutavel do Rc<T> 

    // Rc<T> permitem multiplas referencias a um compartimento de memoria porem nao
    // permitem multiplas referencias mutaveis, pois isso criaria uma brecha para
    // erros/panicos durante a compilacao do programa, por exemplo se mais de uma referencia mutavel 
    // tentasse mudar o valor de um dado no heap iriamos receber como resultado um panic!.

    // Para resolver esse problema um novo SmartP foi criado, usando unsafe Rust 
    // dentro de um protocolo safe para manipular as regras de emprestimo e posse do rust e
    // assim permitir multiplas referencias mutaveis para um dado do heap.
    // O nome desse SmartP e: RefCell<T>

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


fn main() {
    usando_rc();
}

