// Já sabemos muito bem oque são referencias, 
// É a forma que temos de capturam informações de qualquer type dinamico e estatico na memoria
// sem tomar posse de seu valor caso seja um valor dinamico, mas mesmo assim podendo usa-lo.

// Dados no Rust tem tempo de vida ou "lifetimes", eles não são nada mias que o tempo de duração 
// de um valor dentro de em um determinado escolpo, usando um valor generico "a" por exemplo, 
// quando o escolpo de "a" chegasse ao fim seu life time teria terminado e assim "a" seria deletado da memoria
// desta maneita não poderiamos nos refenciar a este valor pois ele já teria sido deletado
// Ou seja a existencia de referencias dependem diretamente de lifetimes.

// Dentro de funções não podemos retornar referencias pois o compilador não consegue saber qual é o lifetime concreto
// da referências que será retornada, para isso precisamos de parametros de lifetime genericos.

// Da mesma forma que funções podem aceitar dados cuja os types são genericos, 
// ela também pode aceitar referencias cuja o tempo seja generico.
// Para demostrar parametros genericos de lifetimes isso iremos usar a função "maior" como exemplo, 
// que irá retornar sempre a maior &str analisada

// O simbolo do generic reference é '  exemplo &'a
// Exatamente como em generics types para usar generics lifetimes em funções usamos a estrutura generic 
// como o usual, usamos ela para expecificar um elemento generico, no caso nossa referencia de lifetime generico <'a>
// o 'a não é nada mais nada menos do que um life time, 
// ou seja a função cria um lifetime proprio e o compara com o lifetime das referencias passadas como parametros e
// e aceita o parametro passado com a condicao que ele tenha um lifetime maior ou igual ao lifetime criado para a func
// assim garantindo que a referencia tenha um lifetime valido durante toda a funcao
fn maior<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        return first.trim();
    }
    else {return second.trim();}
}
// Tinhamos visto que tructs apenas podiam guardar types que n fossem referencias,
// Mas agora que sabemos sobre life times podemos implementar structs que guardem referências, 
// mas para isso precisamos adicionar estruturas de definicao de lifetimes genericos na struct
struct pato<'b> {
    name: &'b str,
}

// Dentro do rust temos regras de elisao, basicamente sao 3 regras que resumem o comportamente 
// do compilador do rust quando se trata de Generics lifetimes, 
// ou seja ela dita quando q o proprio programador precisa ou n 
// especificar o lifetime como generic de uma funcao que possue referencia como parametro ou retorno

// 1 - caso a funcao tenha referencias como parametro, mas n retorne essas referencias o proprio compilador
// compreende a existencia dessas referencias cm diferentes lifetimes dentro da func, 
// e cria automaticamente generic lifetimes para cada um desses parametros 

fn exemplo1(a: &str, b: &str, c:&str) {
    println!("{}, {}, {}", a, b, c);
}

// 2 - caso em uma funcao tenhamos apenas uma referencia cm parametro e retorno, o lifetime dessa referencia 
// e aplicada para toda a func, sendo assim n e necessario expecificar um generic lifetime 
// para essa referencia dentro da func, o proprio compilador ja a implementa
fn exemplo2(a: &str) -> &str {
    return a;
}


// 3 - em caso de metodos de structs e enums, caso o retorno deste metodo seja uma reference "&self",
// o proprio compilador ja demarca o lifetime dessa reference, 
// sendo assim n e necessario expecificar um lifetime para este metodo

struct ex {
    exx: i32,
}

impl ex {
    fn exemplo3(&self) -> &ex {
        return self;
    }
}


// Para criar metodos que necessitem de generics lifetimes 
// apenas precisamos explicitar o generic lifetime dentro do metodo  

struct gg {
    name: String
}

impl gg {
    fn tentativa<'a>(&self, c: &'a str, b: &'a str) -> &'a str {
        return c;
    }
}



fn main() {
    println!("{}", maior("um", "dois"));
    // Temos um tipo de referencia especial que seu lifetime existe durante toda a duracao do programa,
    // Ele e chamado de Static lifetimes, definimos ele usando:
    // & + lifetime simbol ' + static + type
    // Exemplo:
    let sta: &'static str =  "Meu lifetime e inifinto muahahhahah";
}

