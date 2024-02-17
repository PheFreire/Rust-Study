
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//      Introducao RefCell<T>

    // O RefCell<T> funciona da mesma maneira de uma box<T>,
    // pois ambas são SmartP que contem em sí a posse (referencia mutavel) de um de um dado armazenado no heap,
    // ou seja diferente do Rc<T> que possui diversos possuidores e
    // consequentemente ultiliza um contador de multiplas referencias, o RefCell<T> e a Box<T>
    // possuem apenas um dono.

    // A unica diferença entre a box<T> e o RefCell<T> é:
    // A Box<T> tem suas referencia processadas em tempo de compilação (safe rust), ou seja,
    // para que a Box<T> funcione, o programa deve conseguir pre-compilar 
    // a pilha referente as referencias dessa box Box<T> e 
    // garantir que a mesma esteja seguindo todas as regras de borrowing e ownership do Rust, 
    // assim evitando panicos no codigo como por exemplo impedindo que exista mais de um elemento
    // que possua referencia mutavel sobre o valor da Box<T>.
    
    // Já o RefCell<T>, tem suas referencias processadas em tempo de execução (desconsiderando a pilha) (unsafe rust),
    // assim permitindo que mais de um elemento possua referencia mutavel sobre seu valor, 
    // funcionando exatamente como listas de linguagens dinamica, pois já que a regra da existencia de
    // multiplas referencias mutaveis do RefCell<T> é desconsiderada e não é checada na pilha (ultilizando unsafe Rust), 
    // os valores assim como no python serão processados na hora em que forem usados,
    // e caso ocorra algum erro durante a execução do script receberemos um panic!, 
    // sendo assim o RefCell só se preocupa com uma coisa:
    // se durante o tempo de execução as regras de borrowing e ownership estão sendo compridas

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ 


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//       O grande truque (Mutipla referecia mutavel)

    // Já que o RefCell compilado em tempo de execução, se colocarmos
    // um RefCell dentro de um Rc podemos ter multiplos,
    // possuidores de um espaço de memoria do heap (devido ao Rc) 
    // e ao mesmo tempo esses possuidores terão Referencia mutavel a este espaço
    // (Devido a propriedade de processamento em tempo de execução do RefCell)
    // Ou seja teremos um elemento com multiplos donos q podem mudar seu valor
    // Assim como no Python ou no JavaScript
    // Ou seja um Rc<RefCell<T>> é um dado apenas que pode ter multiplos possuidores
    // E que pode ter seu valor alterado por multiplos elementos

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//               Camada de mutabilidade interna

    // A camada de mutabilidade interna é outro recurso que o RefCell<T> nos permite usar,
    // Isto é: poder alocar um elemento com referencia mutavel dentro de um elemento imutavel,
    // isso é possivel pois as referencias de um RefCell<T> e suas propriedades de mutabilidade 
    // são processadas em tempo de execução e por causa disso a suas outra propriedade de poder
    // ser alterada por multiplos elementos faz com que elementos exeternos consigam mudar
    // seu valor internamente mesmo estando em um elemento imutavel

    // Este processo é uma otima abstração de segurança, já que
    // de maneira externa o elemento propenso à camada de mutabilidade interna não pode ser alterado,
    // ou seja o processo de manipulação dos valores desse elemento só podem ser vistos de maneira muito interna 

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//                 Importando o SmartP RefCell<T>
    
    use std::cell::RefCell;
    use std::rc::Rc;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//          Explicação do exemplo pratico do RefCell

    // A Trait "Hard" com o metodo "tryy()" tem o poder de modificar a struct "Ex2"
    // A estrutura "Ex1" poderá receber dentro de sí a estruct "Ex2"
    // A estrutura "Ex1" imutavel e a "Ex2" será mutavel 
    // Usaremos "tryy()" para atualizar a estrutura "Ex2" 
    // Veremos que por "Ex2" ser um "Rc<RefCell<T>>" ele pode ser atualizado 
    // por elementos externos que tenham acesso a ele, 
    // mesmo ele estando dentro de uma estrutura imutavel ("Ex1")

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//          Usando RefCell


    // O enum infinity é uma estrutura que pode conter ou 
    // um elemento que permite multiplos possuidores que podem alterar seus dados 
    // em tempo de execução ou uma instancia dela mesma ou o dado vazio Nop 
    #[derive(Debug)]
    enum Infinity {
        List(Rc<RefCell<i32>>, Rc<Infinity>),
        Nop
    }

    // Importando dados da lista para o escolpo global
    use Infinity::{List, Nop};


    // Criando Trait com o metodo que pode alterar Ex2
    trait Hard {
        fn tryy(&mut self, msg: &str);
    }


    // Criando estruct que receba dados generico que implementem a trait Hard (como s strcut Ex2)
    // Recebe como atributo um SmartP que recebe um dado generico que compila em tempo de execução
    // permitindo multiplas referencias mutaveis :)
    struct Ex1<T: Hard> (Rc<RefCell<T>>);


    // implementando metodo de criação da struct Ex1
    impl<T: Hard> Ex1<T> {
        fn new(dt: Rc<RefCell<T>>) -> Ex1<T> {
            return Ex1(dt);
        }
    }


    // A struct Ex2 recebe a estrutura List para criação de 
    // listas de dados que possuem multipla referencia mutavel
    #[derive(Debug)]
    struct Ex2(Infinity);

    // Impleméntando a trait Hard para a struct Ex2    
    impl Hard for Ex2 {

        /// O metodo Tryy na struct Ex2 adiciona dentro da lista de dados do Ex2
        /// um Infinity::List() com o len() da &str inserida como parametro  
        fn tryy(&mut self, msg: &str) {
            match &self.0 {
                List(dt, _) => {
                    let dado = Rc::new(List(Rc::new(RefCell::new(msg.len() as i32)), Rc::new(Nop)));
                    self.0 = List(Rc::clone(dt), dado)},
                Nop => {self.0 = List(
                    Rc::new(RefCell::new(msg.len() as i32)),
                    Rc::new(Nop)
                )}
            }
            println!("{:?}", self.0);
        }
    }

    fn main() {
        let mut lets1 = Rc::new(RefCell::new(Ex2(Nop)));
        let test2 = Ex1::new(Rc::clone(&lets1));
        lets1.borrow_mut().tryy("pato");
        println!("{:?}", test2.0);
    }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

