// vetores no rust ou "Vec<T>" é um Type que pode guardar infinitos elementos de um msm type
// Ou seja, ao contrario de arrays, Vec<T> n precisam expecificar 
// qnts elementos serão armazenados dentro de si

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //

fn main() {
    // Para criar um vec definimos uma array vec!
        let abc: Vec<i32> = vec![];
    
    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //
    
    // Para colocar valores dentro de uma array Vec<T> usamos o comando .push()
    // Para alterar um Vec temos que cria-lo como um elemento mutavel
        let mut abc: Vec<i32> = [8, 9, 10];
        abc.push(12);
        println!("{}", abc);
    
    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //
    
    // Para chamar um elemento de um Vec<T> podemos chama-lo direto pela sua index ou pelo metodo get
    // A diferença entre chamar pela index e chamar pelo get se encontra 
    // na hora que chamamos uma index inesistente no Vec<T>, 
    // quando isso ocorre, no primeiro caso o retorno é um Panic!, ou seja o programa é finalizado com o aviso de erro fatal
    // No caso do get, se a index chamada não existir, retorna-se um enum Some() com o valor de None
    
    // Vec<T>[index];
        println!("{}", abc[0]);
    
    // Vec<T>.get(index);
        match abc.get(0) {
            Some(a) => {println!("{}", a);},
            Some(_) => {println!("Not exist!")}
        };
    
    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //
    
    // Eu disse que os Vec<T> armazenar conjuntos de dados, mas apenas de um Type, 
    // mas se eu quiser armazenar Types diferentes em um mesmo Vec<T>?
    // Para esse problema, criamos um enum como todos os Types que queremos usar no nosso Vec<T>,
    // definimos este enum como Type do Vec<T> em questão e assim podemos usar os Types definidos no enum dentro do nosso Vec<T>
        enum Solucao {
            Int(i32),
            Float(f64),
            Ss(String),
            Booleam(bool)
        }
    
    
        let finall: Vec<Solucao> = vec![
            Solucao::Float(10.5), 
            Solucao::Int(12), 
            Solucao::Ss(String::from("pato")), 
            Solucao::Booleam(true)
        ];
    
    }