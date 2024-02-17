// O Rust tem seus comandos separados em modulos e isso traz a melhor otimização possivel
// pois só carrega para o ambiente global os comando necessarios que estiverem sendo usados no script
// Sua biblioteca de comandos padrão é o std e para import-la é necessario o comando "use nome_da_biblioteca;"

use std::io;

fn main() {
    
    // Para definir que a variavel é um String é necessario um metodo String
    // Como no python para definir que uma variavel será uma array é necessario o metodo list() ou se for um dicionario o metodo dict()
    // Dentro do Rust é necessario invocar dentro do modulo String o metodo new() que se refere que aquela var é uma String "nova" (vazia)
    let mut palpite = String::new();
    
// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //
    
    // Para conseguirmos capturar informações vinda do terminal durante a execução do program precisamos ultilizar o modulo io (io = Input/Output)
    // Dentro desse mudulo temos a função de leitra padrão "stdin" que captura tudo oque for digitado no terminal
    // para enviar os dados capturados para uma variavel usamos a funçao read_line(), 
    // o read_line() lê o retorno do stdin e o envia para a variavel referenciada em seu parametro
    // Para conseguirmos capturar informações vinda do terminal durante a execução do program precisamos ultilizar o modulo io (io = Input/Output)
    // Dentro desse mudulo temos a função de leitra padrão "stdin" que captura tudo oque for digitado no terminal
    // para enviar os dados capturados para uma variavel usamos a funçao read_line(), 
    // o read_line() lê o retorno do stdin e o envia para a variavel referenciada em seu parametro

    io::stdin().read_line(&mut palpite).expect("Coloca algo");

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //

    // Nós vinculamos palpite à expressão palpite.trim(). O palpite, na expressão, 
    // refere-se ao palpite original contendo a String de entrada do usuário. 
    // O método trim, em uma instância de String, vai eliminar quaisquer espaços em branco no início e no fim.
    // u32 pode conter apenas caracteres numéricos, mas o usuário precisa pressionar Enter para satisfazer o read_line. 
    // Quando o usuário pressiona Enter, um caractere de nova linha é inserido na string. 
    // Por exemplo, se o usuário digitar 5 e depois Enter, palpite ficaria assim: 5\n. 
    // O \n representa uma linha nova, a tecla Enter. 
    // O método trim elimina o \n, deixando apenas 5.
    
    palpite = palpite.trim().to_string();

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //

    // O comando Parse tem o poder de mudar o valor de uma type string para algum type numeral (u, f, i)
    // Já que estamos redefinindo o type de uma var é necessario 2 fatores: 
    // -O uso do "let" pois estamos redefinindo os parametros da variavel
    // -A especificação do type numerico diretamente na varriavel usando "nome_da_var: type numerico"

    // A Expressão mach captura o retorno de uma função antes dela chegar em uma var
    // Após capturar esse retorno ela o analisa e define quais valores substituirão esse retorno 
    // isso claro dependendo do valor que a função estiver retornado
    // podemos usar o match paara sustituir um ".except()"

    let palpite: u32 = match palpite.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Erro!");
            0
        },
    };

    println!("{}", palpite);

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //

    // len retorna qnts compartimentos na memoria um conjunto de dados esta oculpando
    let num = "pato ganso porra";
    println!("{}", num.len());

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //

    // Strings n possuem tamanho definido, seus elementos tem tamanho de byte instavel e dinamicos, sendo assim possuem a memoria heap
    // Por possuirem valores dinamicos dados na memoria heap sempre vao pertencer a um escolpo especifico, 
    // ou seja, n podem ter seu valor copiados para outro escolpo
    // pois se os dados "a" sofrerem alteracoes os dados "a_copy" n vao receber essas alteracoes, 
    // sendo assim so podemos passar dados existentes para outro escolpo ser dermos a posse dele a esse novo escolpo
    // OU SE USARMOS references "&"

    // References ou &var
    // &var significa: "Ao ser chamado retornar o valor atual existente no compartimento de memoria var"
    // Ou seja se a informacao existente no compartimento de memoria abc for "pato" 
    // Ao chamarmos &abc ela vai retornar "pato"
    // Se mudarmos a informacao de abc para "amo patos"
    // Ao chamarmos &abc ela vai retornar "amo pato"
    // Resumo: &var mapea e retorna as informacoes IMEDIATAS existentes no compartimento de memoria var

    // &str e o type correspondente a captura imediata dos dados existentes no compartimento de memoria de uma String na memoria heap
    // Ela captura esses dados e os transforma em informacao estatica 
    // como se tivesse tirado uma foto dos dados da String na memoria heap, ou seja os dados de uma &str sao imutaveis
    // Eles sao basicamente a adaptacao de um String para que ela possa ser armazenada na memoria pilha ao invez do heap 
    // Ao capturar os dados da String e transforma-los em estaticos esses novos dados sao armazenados em um compartimento proprio agora na pilha

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //    

    // Há duas formas de transformar &str em String
    let j = "exemplo";
        j.to_string();
        String::from("oi");
    
    // Anexando dados a uma String
        let mut string_exemplo = String::from("oi");
        // Para anexar dados "&str" a uma String usamos .push_str();
            string_exemplo.push_str(j);
        // Para anexar dados "char" a uma String usamos .push();
            string_exemplo.push('b');

    // Podemos tambem anexar duas Strings usando o operador "+"
    // Porem a concatenação de dados ocorre apenas entre ums String e uma &String
        let s1 = String::from("pato ");
        let s2 = String::from("pata ");
        let s3 = s1 + &s2;
    
    // Para anexar duas Strings também podemos usar o macro format!();
    // Funciona da mesma maneira que um println!(), mas em vez de printar o valor ele o retorna cm String
        let finall = format!("{} -> {} -> {}", s2, s3, string_exemplo);
    
// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //

    // Para se referir a elementos dentro de string pela index temos que tomar muito cuidado, 
    // pois uma String se resume a um Vec<u8> que armazena bytes de caracteres utf-8
    // Porem alguns caracteres expecificos contem mais de um elemento de bytes
    // Por exemplo o caracter "ते" corresponde a uma coleção de dois elemtos de bytes 
    // com isso n podemos chama-lo em uma String usando String[0], pois ते oculpa dois espaços na memoria 
    // String[0] e String[1], para chama-lo podemos usar String[0..3] 
    // assim estaremos chamamando todos as coleçõesde bytes entre o espaço de memoria (index) 0 e 2
    // E assim conseguiremos englobar todas as bytes correspondedntes ao caracter "ते"
        let abc = String::from("ते");
        println!("{:?}",&abc[0..3]);

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //


    // Para chamar elementos dentro de uma String podemos também chamar os caracteres individualmente como Types chars
    // Com isso não teremos que lidar com a burocracia de mexer com gerenciamento de memoria de bytes de um Vec
    // Fazemos isso usando String.chars(); porem esse metodo retorna um iterator()
    let tryy = abc.chars();
    for e in tryy{
        println!("{}", e);
    }
}


