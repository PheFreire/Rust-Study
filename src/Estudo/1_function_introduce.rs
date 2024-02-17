// Para definir uma função em Rust é necessario o uso de "fn" + nome da função + "()"
// Como em outras linguagens podemos adicionar parametros a uma gunção colocando valores dentro de seu parenteses
// O corpo de uma função é definido por "{}" e tudo oq estiver dentro é parte da função

// a função main é a função que é a primeira coisa a ser executada quuando o programa é executado
// A função main não pode ter parametro nem retonar nada 
fn main() {
    // println! não é uma função comum, é um macro um tipo de função especial do rust
    // println! é uma função que escreve seu parametro no terminal 
    // Após uma expressão no Rust é necessario o uso de ";" 
    println!("Hello, world!");
}


// para modularizar o programa podemos criar outras funções seguindo a mesma estrutura da função "main"
// Diferente da função "main" outras funções podem retornar valores e carregar parametros
// Para definir parametros em uma função é necessario colocar o nome dos parametros e definir seus types
// Para que a função possa retornar um valor é necessario expecificar qual é o type do valor que ela pode retornar 
// para fazer essa definição "usamos -> type" após definir parametro da função
fn soma(a: i32, b: i32) -> i32 {
    // Para retornar valores usamos a estrutura "return"
    return a+b;
}