// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//    Introdução Smart Pointers

    // Dentro da programacao existe o conceito de ponteiros que tem a funcao de apontar 
    // a um compartimento da memoria ram, armazenar e acessar dados na memoria heap
    // Dentro do Rust ja vimos a referencia, um ponteiro que possui apenas a funcionalidade 
    // de se referir a um dado armazenado na memoria, sem tomar posse dele

    // Smart pointers são ponteirs q direfentemente das refererencias possuem a posse de seu valor do heap,
    // podendo também dividir esta posse, ele faz isso guardando não apenas o valor que possue,
    // mas tambem a sua quantidade de donos e deletando e se auto deletando quando sua quantia de donos for 0,
    // chamando a "drop trait" (a veremos mais tarde)

    // Ja vimos alguns types de Smart Pointers como os Vec<T>, HashMaps e as Strings ou seja
    // estruturas que apontam para espacos na memoria heap e possuem posse de seus dados,
    // podendo manipula-los dentro dos limites das regras de posse e emprestimos de valores heap do rust


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//     Qnd usar smart pointers  
    
    // Caso 1-
        // Usar dados que n sabemos o tamanho em uma situacao na qual 
        // precisamos saber o tamanho exato do dado (exatamente o caso das referencias)
    
    // -------------------------------------------------------
    // Caso 2-
        // Transferir a posse de uma grande quantia de dados tendo a certeza q os dados n serao copiados 
        // (usa box<T>). 
        // De maneira mais detalhada: transferir a posse de uma grande quantidade de dados da pilha 
        // é uma tarefa pesada, pois dados da pilha não podem ter sua posse transferida, 
        // neste caso a posse do valor não é tranferida, e sim copiada de um lado da pilha para outro.
        // Manipular uma grande quantia de elementos da pilha simultaneamente para realizar está copia   
        // torna está operação custosa e lenta.
        // Para evitar isso colocamos os dados da pilha q queremos manipular em um unico smartP. 
        // Com isso pelas regras de posse de dados do heap podemos garantir que os dados não serão copiados
        // durante nenhum processo mas sim terão suas posses transferidas.
        // E fazendo este processo tb resolvemos outro problema que deixava a operação lenta, 
        // o de manipular muitos elementos simultaneamente na pilha, e agr manipulamos apenas um elemento, o &SmartP
    
    // -------------------------------------------------------
    // Caso 3-
        // Quando você quer possuir um valor e só se importa com a trait que ele implementa, e n liga para seu type

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  Smart Pointers nativos do Rust
    
    // Podemos usar os smart pointers nativos do Rust como:
    // Box<T>: Permite alocar valores na memoria heap
    // Rc<T>: Permite gerenciar posse multipla de referencias
    // Ref<T> e RefMut<T>: acessados através de RefCell<T>, um type q aplica as regras de empréstimo 
    // em tempo de execução em vez de em tempo de compilação

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//  Funcionamento Smart Pointers

    // Smart pointers são criados a partir de structs (ou seja classes) e obrigatoriamente possuem a trait deref
    // A trait deref q cria o comportamento dos smart pointers, ou seja, 
    // é ela que especifica que os valores dentro de um smart pointer devem ser armazenados no heap e
    // que também quando invocado algum valor do smart pointer, o valor deve ser retornado como referência

    // Também temos a DerefMut que funciona exatamente como a trait Deref, 
    // porem retorna os valores do SmartP como referencias mutaveis ao invez de referencias comuns

    // Como smartP disponibilizam seus comportamentos como traits, podemos usa-las e personaliza-las para
    // criar nossos próprios smart pointers

    // Smart pointers tambem possuem as Drops traits,
    // Elas nos permite manipular os eventos que acontecerao 
    // apos uma instancia de um Smart Pointers sair do escolpo ou, 
    // qnd a quantidade de donos do smart pointer for 0
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//      Criando uma Box
        
    fn chamando_box() {
        // Invocando uma Box<T> e colocando um u8 dentro do heap
        // Agora mesmo sendo um u8 ele tem propriedades de posse 
        let heapp = Box::new(5);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Criando meu proprio Smart pointer

    // Quero que meu SmartP possa armazenar listas de qlqr type,
    // para isso irei declarar o dado que pode ser recebido cm um generic type
    struct Mysmart<T> where T: std::fmt::Display {
        data: T   
    }

    impl<T> Mysmart<T> where T: std::fmt::Display {
        fn new(dt: T) -> Mysmart<T> {
            Mysmart{data: dt}
        }
    }

    // Para transformar minha struct em um SmartP preciso implemetar a trait Deref
    use std::ops::{Deref};
    impl<T> Deref for Mysmart<T> where T: std::fmt::Display {
        // Como em todo SmartP precisamos expecificar qual type q poderemos armazenar 
        // Para isso a trait deref nos permite especificar o valor do atributo Target
        type Target = T;

        // Dentro da Trait deref temos o method deref, é nele onde especificamos
        // como q nosso SmartP retornará os seus elementos armazenados
        // Mas como já explicitado previamento, 
        // um SmartP obrigatoriamente retornará seus elementos em formato de referencia
        // (Isso chega a ser obvio já que estamos mechendo com dados no Heap)
        fn deref(&self) -> &T {
            // Para retornar uma referencia fo primeiro valor contido na lista do nosso SmartP retornamos:
            &self.data
        }
    }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//           Drop Trait
    
    // Dropt Trait como já mencionada, é a trait que permite personalizar 
    // os eventos que irão ocorrer quando um elemento da lista do SmartP for 
    // desalocado da memoria (sair do escolpo)
    // O metodo drop é automaticamente chamado quando o valor sair do escolpo
    // Então não precisamos chama-lo manualmente
    
    // implementando a drop trait no meu SmartP
    use std::ops::Drop;
    impl<T> Drop for Mysmart<T> where T: std::fmt::Display {
        fn drop(&mut self) {
            println!("O valor {} saiu do escolopo!", self.data);
        }
    }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


fn main() {
    let mut abc = Mysmart::new("pato");
    println!("{:?}", abc.deref());
    println!("{:?}", abc);

}


