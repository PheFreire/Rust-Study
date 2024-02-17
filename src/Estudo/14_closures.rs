// No rust podemos guardar funcoes como valores que podem ser passados como parametros e ate retornados de uma func
// Essas chamadas funcoes anonimas ou closures podem ser armazenadas dentro de variaveis,
// receber parametros, ter corpo e realizar eventos como uma funcao normal
// são executadas quando algum valor é dado para ela como parametro
// Para criar funções anonima definimos usando "||" uma área para receber parametros
// tudo oque estiver dentro de || é um parametro da função anonima
// o funcionamento da função anonima será definido após a definição dos parametros

fn closures_XD() {
    // Abaixo temos uma função anonima que retorna o parametro entregue a ela + 1 
    let abc = |x: i32| x+1;
    println!("{}", abc(2));

    // Abaixo temos uma função anonima que retorna se o parametro entregue a ela é positivo ou negativo 
    let exemplo = |x: i32| -> bool {
        x%2 == 0
    };
    println!("{}", exemplo(6));
}

// Podemos armazenar closures dentro de structs e quando ela é chamada 
// seu retorno ja é automaticamente capturado pela struct, mas para isso, por closures n serem types 
// teremos que definir o type da closure como generic para assim a struct conseguir mante-la
// tambem precisaremos de uma trait para que possamos configurar os types de parametro e retorno da nossa closure

// Usamos a trait Fn e especificamos o type do parametro e o type do retorno da closure que vamos definir
// lembrando que o generic type nesse caso "T" ele esta se referindo a nossa closure
// O resultado da nossa closure ira para o atributo option value 
struct cap<T> 
where T: Fn(&str) -> String {
    ff: T,
    value: Option<String>,
}


// Implementando metodos para poder adicionar um closure na nossa struct
impl<T> cap<T> 
where T: Fn(&str) -> String {

    /// adiciona uma closure a nossa struct e retorna a struct atualizada
    fn new(f: T) -> cap<T> {
        cap {ff: f, value: None}
    }

    /// Metodo que executa e retorna resultado da nossa closure para algum parametro
    /// e retorna o resultado no atributo value
    fn event(&mut self, arg: &str) -> String {
        match &self.value {
            Some(v) => v.to_string(),
            None => {
                // self.ff se refere a nossa closure, e a chamamos dentro de parenteses
                // pois queremos nos referir a ela, e não executa-la
                let finall = (self.ff)(arg);
                self.value = Some(finall.clone());
                finall
            }
        }
    }

}

fn main() {
    // Sabemos q closures mesmo sendo funcs podem interagir com as variaveis do
    // escolpo no qual foi envocada, e para isso todas as closures sempre estao submetidas 
    // a uma trait que dita o comportamento dessa interacao.ys
    // Essas closures sao setadas pelo proprio compilador (menos se a closure for setada dentro de uma struct)
    // Temos os seguintes traits:

    // -Fn(&self) -> esta trait dita que as variaveis do escolpo serão usadas na closure como uma referencia
    // e não terá a posse do valor

    // -FnMut(&mut self) -> essa trait passa as variaveis do escolpo para closure como uma referencia mutavel, 
    // assim mesmo sem ter posse do valor ainda poderá altera-lo

    // FnOnce(self) -> essa trait passa a posse das variaveis do escolpo para dentro da closure, 
    // ou seja essa closure só pode ser chamada uma unica vez  
    
    // Temos também a particula "move", que obriga o compilador a capturar a posse das variaveis que
    // forem sitadas ou usadas na closure
    // Quando a posse de um valor é passado para um outro local internamente no Rust oque está ocorrendo
    // não é nada mais nem nada menos que um move 
        let mut batata = String::from("Value: ");
        let abc = move |x:String| format!("{}{}", batata, x);
        println!("{}",batata);
}