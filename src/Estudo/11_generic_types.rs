// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= /// 
/*

##  Geric types são como parametros indefinidos que podem ser substituido pelo type de qualquer valor

##  Como em parametros de funções nos quais podemos substituir o parametro por qualquer valor de um type pre-expecificado, 
##  generics são quase a mesma coisa, sua diferença consiste da possibilidade de substituir não apenas a informação do valor,
##  mas seu type também.

##  Usamos generics quando mechemos com Enums como Option<T> e Result<T> ou quando trabalhamos com HashMaps<T> e Vec<T>, 
##  o "<T>" é o generic type, sendo que a letra dentro de si é um parametro que futuramente será substituido 
##  pelo type de algum valor que iremos inserir.

##  Definir um generic tem tudo a ver com a memoria Heap, pois da mesma forma que Vec<T> e Enums<T>,
##  estamos dizendo para a memoria preparar um espaço em si no qual só poderemos definir o seu type e tamanho 
##  qnd o programa estiver sendo executado

## Colocamos o generic type diretamente no nome da estrutura que queremos que possa receber valores de diferentes types,
## fazemos isso para que o compilador quando executar o programa, já definir que a estrutura em questão
## deve ter seus dados compilados diretamente na memoria heap

*/

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //

/// Structs de generic types são structs q não possuem types definidos para seus atributos até que a struct seja criada.
/// Para definir a struct como generic, chamamos o generic type na criação da estrutura logo após o nome da struct,
/// depois apenas colocamos os parametros da struct types dentro dos atributos da struct
struct Duvida<F,I> {
    numero1: F,
    numero2: I,
}

/// Podemos implementar generic type dentro de metodos impl
/// Isso faz com que o compilador interprete que dentro do metodos impl estaremos lidando com generic types
impl<F,I> Duvida<F,I> {
    fn copiar(&self) -> (&F,&I) {
        (&self.numero1, &self.numero2)
    }
}

// Metodos de um impl podem conter e retornar generics types tb
impl<F, I> Duvida<F, I> {
    fn mistura<V, W>(self, other: Duvida<V, W>) -> Duvida<F, W> {
        Duvida {
            numero1: self.numero1,
            numero2: other.numero2,
        }
    }
}

/// Podemos criar metodos que só podem ser usado caso o type dos generics types tenham sidos substituidos por um type expecifico
/// Nesse caso não precisamos de colocar o generic type durante a criação da estrutura impl, 
/// precisamos apenas definir os types obrigatorios para a execução dos metodos
impl Duvida <f64, i64> {
    fn return_as_int(&self) -> i64 {
        return self.numero1 as i64 + self.numero2
    }
    
    fn return_as_float(&self) -> f64 {
        return self.numero1 + self.numero2 as f64
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //

/// Enums de generic types
/// Para definir uma enum cm generic, como nas structs chamamos o generic type após seu nome,
/// depois apenas substituimos os valores dentro de sí pelos parametros do generic type
enum Exemplo<P,I> {
    Pato(P),
    Pata(I)
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //

//  Funções genericas são funções que podem receber parametros e retornar valores de types genericos ou seja,
//  funções que não sabem qual é o type do valor que irão receber e retornar, até que a função seja chamada.

//  Colocamos o generic type após o nome da função, no type do parametro e no type do retorno da função
//  É obrigatorio o uso de uma Trait para que funções genericas funcionem, então a função abaixo não ira funcionar 

fn printa_type_generico<G, U>(generico: &G, generico2: &U) -> G {
    println!("{}", generico);
    println!("{}", generico2);
    generico
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //


fn main() {
    let abc = Duvida {numero1:12, numero2:20.5};
    let pato: Exemplo<i32, i32> = Exemplo::Pata(12);
    let pato: Exemplo<i32, String> = Exemplo::Pato(12);
}