 use std::io::stdin;
 use std::cmp;
 
 pub fn estruturas() {
    println!("Quantas letras tem o alfabeto?: ");
    let mut abc = String::new();
    stdin().read_line(&mut abc).expect("Erro!");

    let abc: i32 = match abc.trim().to_string().parse() {
        Ok(num) => num,
        Err(_) => {println!("Erro!");0}
    };
    
    // As estrutura if funciona normalmente realizando oq está em seu corpo qnd o resultado da estrutura logica for True
    // As estruturas logicas igual, diferente, ou, e, maior, menor, maior ou igual, menor ou igual e não são respectivamente
    // ==, !=, ||, &&, >, <, >=, <= e !
    if abc == 26 {
        println!("Esse é o alfabeto!");
    }
    else if abc == 52 {
        println!("Esse é o alfabeto com minusculas e maiusculas!")
    }
    else {
        println!("Esse alfabeto está errado!");
    }

    // a estrutura loop permite criar loops infinitos como em um While True
    // Tudo oque estiver dentro do seu corpo estará em loop
    // Para quebrar o loop infinito usamos o comando break;
    loop {
        println!("Estou em loop hihihi");
        break
        }
    
    // Loops podem estar dentro de uma variavel caso o loop retorne um valor
    // Para retornar um valor usando loop é necessario usar "break (valor que ira retornar);"
    let mut a = 0;
    let varloop: i32 = loop { 
        a += 1;
        if a == 26 {
            break a
        }
    };

    // A função cmp ("compare") compara de maneira numerica o valor entre 2 vars e retorna 
    // se o valor da var principal é maior menor ou igual ao valo da var em seu parametro
    match varloop.cmp(&abc) {
        cmp::Ordering::Equal => {println!("O valor da varival varloop tem o mesmo tamanho de abc");},
        cmp::Ordering::Greater => {println!("O valor da varival varloop é maior do que o de abc");},
        cmp::Ordering::Less => {println!("O valor da varival varloop é menor do que o de abc");}
    };

    // for loop, para cada valor entregue para o for por um iterator, for irá repetir as instruções de um bloco de codigo
    // Iterator é a estrutura que toda vez que é invocada retorna algum valor diferente

    // Podemos usar o iterator range no qual o for passa em todos os numeros expecificados entre (numero1..=numero2)
    // Podemos usar tambem o rev() para os numeros serem ditados de tras para frente 
    for num in (1..=10).rev() {
        println!("{}", num);
    }
    
    // Para poder usar uma array vec com o for é necessario um metodo que repasse os valores dentro da variavel como um iterator
    // O nome desse metodo é iter()
    // Para que retorne a index do elemento que está sendo lindo dentro da array vec usamo enumerate()
    let animais = vec!["Coelho", "Rã", "Pato"];
    for (ind, a) in animais.iter().enumerate() {
        println!("O nome do animal na index {} é {}!", ind, a);
    }
}


