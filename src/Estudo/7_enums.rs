
// No rust uma variavel pode apenas receber um tipo de type fixo
// Porem em diversos casos precisamos fazer com que a mesma variavel possa ter types diferentes
// Ou em outros casos em uma funcao por exemplo 
// tem vezes que nao sabemos qual o type que iremos colocar em seu parametro
// No caso de uma funcao que captura o ip temos o ipv6 e o ipv4 
// podemos usar uma enum e deixar explicitos dois valores possiveis para o type dessa enum
// Um para o ipv6 e outro para o ipv4


// Para criar uma enum usamos a seguinte estrutura:
//  enum "nome da enum" {
//      nome_do_type(valor referente ao type em questao),
//      nome_dos_outros_types(valor referente ao type em questao)
//  }

Ipp {
    Ipv4(u32, u32, u32, u32),
    Ipv6(String)
}

// Exemplo 2
enum Mensagem {
    // Sair não tem nenhum dado
    Sair,
    // Mover contém uma struct anônima
    Mover { x: i32, y: i32 },
    // Escrever contém uma única String
    Escrever(String),
    // MudarCor contém uma tupla com três valores do type i32
    MudarCor(i32, i32, i32),
}


// Como em structs podemos criar metodos para enums usando impl
impl Ipp { 
    fn printando_ip(&self) {
        match self {
            Ipp::Ipv4(t,t1,t2,t3) => {println!("{}.{}.{}.{}",t,t1,t2,t3);},
            Ipp::Ipv6(t) => {println!("{}",t);},
        }   
    }
}


// Enum option
// O Rust n tem um valor null (nulo) pois valores nulos contem muitos erros e problemas
// Sabendo disso como vamos por exemplo criar parametros em funcoes que podem ou nao ter algum valor
// Para isso temos uma enum do proprio Rust Chamada Option
// O estrutura do Option (lembrando que o option e do proprio Rust entao nao e necessario cria-lo):

//  enum Option<T> {
//      Some(T), // algum valor
//      None, // nenhum valor
//  }

fn soma(a: i32, b: i32, c: Option<i32>) -> i32 {
    match c {
        Option::Some(t) => {return a + b + t;},
        Option::None => {return a + b;}
    };
}


// Curiosidade sobre o match, podemos apenas adicionar um "_" 
// se nao quisermos expecificar os restos dos valores cobertos pelo match

//  let algum_valor_u8 = 0u8;
//  match algum_valor_u8 {
//      1 => println!("um"),
//      3 => println!("três"),
//      5 => println!("cinco"),
//      7 => println!("sete"),
//      _ => (),
//  }


fn main() {
    let abc = Ipp::Ipv4(162, 039, 436, 73);
    abc.printando_ip();
    println!("{}", soma(12, 13, Some(2)));
}