// No rust variaveis são de maneira padrão imutaveis
// Para criar uma variavel é necessario usar a estrutura: 
// "let nome_da_var: type(não obrigatorio) = valor_da_var;"
// O Rust consegue detectar em 99% dos casos qual é o type de uma variavel baseado em seu contexto

fn main(){
    // O rust detectou o Type dar var
    // A var abaixo é imutavel
    let first = "minha primeira var";

    // A variavel abaixo teve seu Type pre definido
    let numero: i32 = 18;

    // Para que uma var seja mutavel é necessario o uso do termo "mut" apos o termo "let"
    let mut inteiro = 128;
    inteiro = 19;

    // Para constantes no rust é necessario o termo const e é obrigatorio ter seu Type pre definido e usar de letra maiuscula  
    const CONSTANTE: i32 = 12;

    // para definir uma array seguimos a estrutura de crição de variavel
    // Mas dde maneira obrigatoria devemos definir o type aceito na array e também o seu tamanho
    let array1: [i32;3] = [12, 13, 14]; 
    
}   