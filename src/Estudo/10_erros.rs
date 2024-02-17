use std::fs;
use std::io;

// Erros recuperáveis são erros razoável que podem ser reportados ao usuário 
// pra tentarem realizar a operação novamente e resoluver o erro, exmplo: erro de arquivo não encontrado. 
// Erros recuperaveis são tratados pela estrutura enum "Result<>"

// Erros irrecuperáveis são bugs que dão panico no sistema e acabam com o funcionamento do msm
// exemplo: tentar acessar uma localização além do fim de um array
// Erros irrecuperaveis não podem ser tratados e são chamados de "panic"
// O unico jeito de consertar um panic é manipulando o ambiente para que seja impossivel ocorrer um panic
// Ou seja USA CLEAN CODE FDP

fn main() {
    // Panic é freagem abrupta da execução de um script quando um erro causa panico no codigo
    // No funcionamento interno do Rust o panic é constantemente usado, 
    // para que erros não causem funcionamentos irregulares ou possiveis vulnerabilidades no codigo
    // Por exemplo se tenatrmos acessar uma index inesistente de um dado heap, 
    // ao invez de retornar os dados existentes no compartimento da memoria requisitado,
    // o proprio Rust retorna um panic, ele faz isso chamando o macro panic!() 
    // que é a func no rust que consegue invocar e mapear erros de panico
        let abc = vec![1, 2, 3, 4, 5];
        println!("{}", abc[10]);

    // Também podemos usar o macro panic! para criar erros no nosso codigo, 
    // isso seria util caso estajamos criando uma biblioteca por exemplo 
    // O parametro que colocarmos no macro panic!() será printado no terminal cm a explicação do erro
        panic!("Hasta luego");

    // Como já foi dito o panic não apenas aborta o programa mas também mapeia o erro
    // Sendo assim podemos rastrear oque causou o panic e ir atras de concertar o erro
    // Mesmo que o erro esteja acontecendo em um modulo dentro de outro script e apenas estamos invocando uma func desse script
    // o panic rastreia o erro e retorna a linha que esta causando o abortamento 
    // Temos uma variavel local do Rust chamada RUST_BACKTRACE ("rastro rust")
    // esta variavel é uma lista de todas as funções que foram executadas até chegar no nosso panic
    // Podemos acessa-la ativando o modo debug ao executar o script em questão
    // Se tivermos um panic e não ativarmos o modo debug o compilador nos retornará uma msg 
    // dizendo que temos que ativar o modo debug para acessar o RUST_BACKTRACE
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

    // Existem os erros trataveis, funcionam como uma enum e são chamados de result podendo retornar
    // Err<E> no caso de um erro, sendo o valor "E" uma string expecificando o motivo do erro
    // ou Ok<T> sendo T a resposta da função caso a função não dê certo
    // Ou seja, existem funções que só retornam o type Result<> com Err e Ok
    // Como a enum Option, não precisamos de "Result::" para se referir ao Ok<T> ou ao Err<E> 
    // pois a estrutura Result como a Option já estão importadas para o escolpo global do Rust 
    // exmplo do que não precisamos fazer -> Result::Ok(T)
    // Podemos fazer apenas Ok(resposta) ou Err(erro)

    // A função "std::fs::read" retorna um result com o handler do file aberto ou um erro de arquivo não encontrado
    // Como uma Option podedmos tratar esse erro com match
    let data = match std::fs::read("./Exercicios/data.txt")  {
        Ok(resp) => {println!("Arquivo encontrado");resp},
        Err(er) => {panic!("Arquivo não encontrado!\nErr => {}", er);}
    };


    // Podemos também decidir tratar erros diferente de maneiras diferentes
    // Fazemos isso usando a estrutura if + o metodo ".kind()" no erro que ira nos dar o nome do erro,
    // com a junção dessas estruturas conseguimos saber o nome do erro e diferencia-lo dos outros erros possiveis
    // Lembrando que se quisermos saber o nome do erro expecifico podemos forçar o erro e o terminal ira printar o nome do erro
    let data = match fs::File::open("./Exercicios/data.txt") {
        Ok(file) => file,
        Err(ref error) if error.kind() == io::ErrorKind::NotFound => {
            match fs::File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {panic!("Tentou criar um arquivo e houve um problema: {:?}",e)},
            }
        },
        Err(error) => {
            panic!("Houve um problema ao abrir o arquivo: {:?}",error)},
    };

    // Podemos também nos referiri diretamente ao dado dentro do Ok<T>, 
    // e caso o resultado for Err(E) ele automaticamente retorna um panic!
    // Para isso usamos o metodo ".unwrap();" ou ".expect();"
    // A diferença entre esses dois metodos é que o except nos permite escolher qual será a menssagem printada pelo panic!
    let data = std::fs::read("./Exercicios/data.txt").unwrap();
    let data = std::fs::read("./Exercicios/data.txt").expect("FODEU ERRO ERRO ERRO!");

    
}


// Podemos também criar funções que retornem result<>
fn tryu(path: &str) -> Result<String, std::io::Error> {
    // Retornar erros no Rust é tão comum que temos um unico operador que funciona como um match
    // O operador "?", que retorna o erro caso o resultado dê Err(E), 
    // funciona literalmente como um return, retornando o Err(E) diretamente e caso o Result dê Ok(T)
    // Ele simplesmete retorna o valor do Result para dentro do escolpo (n cm um return, retorna mas sim para o escolpo da func)
    let abc = std::fs::read_to_string(path)?;
    Ok(abc)
}