// Traits são estruturas que nos permitem expecificar comportamento para diferentes tipos de estruturas e dados 
// As traits nos permite usar um mesmo metodo em structs diferentes que contenham types diferentes
// desde que essa função funcione para ambos os diferentes types das diferentes structs em questão

struct lagoa {
    qnts: i32,
    nome: String
}


trait pato {
    fn japa(&self) -> &str;

    fn print_japa(&self) {
        println!("{}", self.japa());
    }

    fn exemplo(&self) {
        println!("Você só me verá caso vc n tenha alterado a estrtura do metodo que eu estou")
    }
}


// Uma trait é como se fosse um bau cheio de items (no caso do trait, metodos),
// Quando implementamos uma trait a uma struct, 
// é cm se estivessemos dando a chave deste bau para essa struct, 
// sendo que agora ela pode acessar todos os items dentro desse bau (os metodos declarados na trait)
impl pato for lagoa {
    // Ao implementarmos o Trait dentro de uma struct podemos alterar seus metodos caso seja necessario
    // Como por exemplo, para usarmos o metodo "print_japa()", precisamos expecificar o retorno do metodo "japa()" 
    // para adequa-lo á struct em questão.
    fn japa(&self) -> &str {
        &self.nome
    }
}


fn main() {
    let abc = lagoa {qnts:2, nome: String::from("gata")};

    // No caso de não alterarmos o corpo de um metodo da trait ao implementarmos ele em uma struct,
    // ele se manterá com o mesmo funcionamento explicitado em sua trait, 
    // por exemplo o metodo "printar_japa()", que nos não alteramos seu corpo ao o implementamos na struct lagoa
    // e ele manterá seu funcionamento defaut
    abc.print_japa();
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

struct Duck {
    name: String,
}

// Podemos usar traits para definir comportamentos de types genericos dentro de funções e metodos
// Quando definimos types genericos dentro de funções e metodos temos que definir quais tipos de
// funções e metodos aquele type generico é capaz de implementar, exemplo: 
// a String pode implementar o metodo "from();", o Vec<> pode implementar o metodo "push();" etc...
// Para definirmos que tipos de metodos aquele type pode implementar, 
// criamos um trait e colocamos dentro desse trait 
// todos os metodos que o nosso type generico tem acesso a implementar


trait IsAnimal {
    // Quero que meu generic type possa implementar o metodo "die();"
    fn die(&self) -> ();
}

// Implementando comportamento usando where
fn generic_bla<T>(ducks: &[T]) -> T
where
// Trait que eu estou querendo que o generic type tenha acesso aos metodos
    T: Clone + IsAnimal,
{
    ducks[0].die();
    ducks[0].clone();
}


    