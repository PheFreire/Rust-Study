// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//      Armazenamento de dados

    // Todo programa armazena seus dados na memoria ram do computador
    // a mempria ram é a parte do computador responsavel 
    // por armazenar dados temporarios usados apenas durante a execução de um programa
    // por exemplo: variaveis, quando iniciamos um script e declaramos uma variavel,
    // o computador armazena o valor dessa variavel na memoria ram e quando os valores não
    // estiverem mais sendo ultilizado o espaço na memoria ram que armazena
    // esses valores são desalocados

    // Ao armazenar um dado, a memoria ram deve preencher um certo "formulario" obrigatorio 
    // referente ao dado que está sendo armazenado, sendo os campos desse formulario:
    // -Endereço daqule slot de memoria dentro da memoria RAM (ID)
    // -Tipo primitivo do dado
    // -Tamanho da memoria que está sendo requisitado para alocar o dado
    // -Tamanho do dado que será armazenado
    // -O dado que será armazenado
    // -O nome das variaveis que tem posse sobre o dado
    // Completando o formulario acima conseguimos armazenar o dado na memoria Ram,
    
    // Agora, antes de prosseguir temos dois problemas:
    // 1-Quando que devemos desalocar um dado da memoria RAM
    // 2-Como armazenar elementos que possuem dados dinamicos, ou seja,
    // variaveis que possuem valor mutavel como Vec<T> e Strings e que por 
    // não possuir tamanho fixo não podem preencher o formulario de armazenamento

    // Para responder as duas perguntas precisamos entender os protocolos que ditam
    // como que os dados serão armazenados e acessados: 
    // O protocolo Pilha e o protocolo Heap


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//              Pilha

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //          Introdução

        // A pilha é um protocolo que monta e analisa a
        // estrutura dos dados do programa antes do mesmo ser executado
        // por a pilha já ter tudo estruturadinho antes mesmo 
        // do programa ser executado, ele é muito rapido 
        // e também pode detectar erros no codigo antes mesmo da execução do programa
    
        // E pelo fato da pilha estruturar tudo antes mesmo da execução do programa,
        // os dados dentro dela não podem ser dinamicos, ou seja:
        // Para armazenar dados na pilha o formulario de armazenamento da memoria ram
        // deve estar 100% completo sem problemas, ou seja é obrigatorio saber o
        // tamanho, o type e todas as outras informações do dado que será armazenado.

        // Cada escolpo possui sua propria pilha e dentro dela os dados que foram declarado
        // dentro de sí, sendo assim quando esse escolpo acaba, os dados
        // referente aquela pilha são desalocados da memoria
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //        Montando a pilha

        // Os dados na pilha são montados de maneira empilhada
        // sendo assim similar à uma pilha de pratos, cada elemento lido é
        // armazenado na memoria ram e é organizado
        // como base do outro ou seja, é organizado um em cima do outro em ordem de chegada.
        // Exemplo:
        fn exemplo() {
            let a = 10;
            let b = 20.4;
            let c = "pato";
        }
        // A pilha dos dados acima é montada da seguinte maneira:
        // 3-c - O dado "c" por ter sido o ultimo a ser lido fica no topo da pilha
        // 2-b
        // 1-a - O dado "a" por ter sido o primeiro a ser lido é a base da pilha

        
        
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //         Leitura da pilha

        // Após a estruturação da pilha, os dados são lidos do topo
        // da pilha até a base, pense sempre em uma pilha de pratos, 
        // não se retira os pratos da base, retiramos eles na de cima para baixo
        // e é exatamente nesta ordem que os dados serão analisados após a montagem da pilha

        // Nessa analise, conferimos se as variaveis chamadas 
        // realmente existem e foram declaradas dentro contexto que foram chamadas, 
        // conferimos se elas possuem o type, e o tamanho expecificado na pilha  
        // e também pre-processamos quando os elementos devem ser desalocados
        // desta maneira conseguimos pre-compilar a pilha e seus possiveis erros
        // antes de sua execução
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//               Heap

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //           Introdução

        // De maneira direta, para armazenar dados com tamanho e types desconhecidos usamos o heap
        // O heap completamente ao contrario da pilha não tem seus dados pre compilados,
        // ou seja seus dados são acessados e analisados 
        // em tempo de execução (como em inguagens dinamicas)
        // por esse motivo o heap é muito mais lento que a pilha

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //          Funcionamento
        
        // Ao em vez de alocar um elemento em um compartimento na memoria que possua exatamente 
        // o tamanho do dado que será armazenado, o heap por não saber o tamanho exato do
        // dado que ele ira armazenar, ele reserva um espaço na memoria que serve como capacidade maxima
        // por exemplo: o heap reserva um espaço na memoria que consiga armazenar até 128 bytes,
        // e aloca um dado dinamico dentro dele, este dado vai consumindo aquele espaço,
        // e caso o ultrapasse, o heap apenas aumenta a memoria acoplando mais um slot da memoria
        
        // o heap nunca retorna o valor de um elemento alocado em si,
        // ao invez disso, ele retorna um pointer, que é o endereço 
        // do slot da memoria onde você pode encontrar o dado do elemento que vc quer acessar
        // O nome pointer vem exatamente do fato de ser um valor que "aponta" para um espaço da memoria
        // Pointers por serem apenas um texto estatico que possui um endereço de memoria,
        // podem ser armazenados no Heap, pois dentro dele possui um tamanho fixo e estatico.
        // E é dessa forma que dados no heap podem interagir com dados na pilha
        // e manterem o protocolo de segurança do Rust

        // Existem dois tipos de Pointers: 
        // O pointer mais comum e importante: o reference "&"
        // E os pointers mais avançado que serão vistos nos capitulos 17, 18, 19 e 20
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //    Borrowing: References "&"
    
        // O pointer "&" ao ser invocado, ele vai até o local na memoria
        // o qual ele aponta e copia os dados dinamicos existentes lá dentro como
        // especie de "foto" 
        // Ou seja ele copia os dados que antes eram dinamicos
        // em versões estaticas referentes ao momento da captura
        
        // E agora que o valor já não é dinamico, eles já podem ser armazenados na pilha
        // sendo assim essas versões estaticas desses dados dinamicos são retornados para a pilha
        // e agora já podem ser usados normalmente como elementos da pilha
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //     Regra de ownership (Posse)

        // Caso mais de um elemento tenha a posse de um compartimento no heap
        // teremos problemas pois os dados do heap são analisados em tempo de execução
        // dessa forma se dois elementos tiverem posse desse compartimento não temos 
        // um controle previo de que dados são usados, deletados e movidos,
        // assim poderemos ter diversos erros voltados a essa falta de controle.

        // Por exemplo: um elemento A que tem a posse de um compartimento do heap deleta um dado
        // existente neste compartimento, em quanto simultaneamente o elemento B
        // tenta acessar esse dado que foi deletado,
        // caso isso aconteça receberemos um erro pois tentamos acessar um dado inexistente.
        
        // Outro erro que iria ocorrer seria voltado a
        // quando o escolpo que contem o pointer referente a esse compartimento do heap acabasse,
        // a pilha iria detectar que o escolpo do pointer em questão já não existe mais e
        // assim ele iria desalocar o compartimento do heap, porem se dois elementos tivessem
        // posse desse compartimento, esses dois elementos iriam tentar desalocar 
        // o compartimento quando o escolpo acabasse e assim receberiamos um erro de 
        // tentatica de desalocar um compartimento que não existe 

        // Para que isso não aconteça o Rust implementou o sistema de posse no qual apenas
        // um elemento possui pemissão para alterar e gerencia o alocamento e desalocamento
        // de um compartimento do heap

        // ou seja se tentarmos passarmos a posse de um compartimento do heap de uma var para outra,
        // a primeira var será automaticamente desconsiderada e o compartimento do heap será
        // passado para a outra var

        // Ou seja se quisermos passar elementos do heap como parametro ou queremos copia-los para
        // outras variaveis usamos a referencia a está variavel

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


fn main() {
    // Types que possam ser alocados na pilha possuem um comportamento (trait) chamado "copy"
    // Quando uma variavel possui um dado cuja o type possua essa trait, 
    // esta variavel se passada como parametro de uma func ou tendo seu valor adquirido á outras variaveis ,
    // mantendo seu valor na variavel incial e apenas copia o seu valor para esses novos escolpos 
    // (ultilizando o metodo "clone()" disponivel na trait "copy")
    let abc = 10;
    let clonando = abc;
    println!("{} -> {}",abc, clonando);

    
    // Exemplo de transfusão da posse de um compartimento do heap de uma var para outra
    let posse = String::from("exemplo");
    let movevar = posse;
    println!("{}", movevar);
    println!("{}", posse);
    
    // E no caso de ele ser chamado como parametro em uma função ele deixa de existir no seu bloco de codigo inicial e passa a existir apenas no bloco função que o chamou
    // Ou seja ele perde a posse do valor
    let hold_var = String::from("pato");
    tomar_posse_da_var(hold_var);
    println!("{}", hold_var);

    // Uma das maneiras de ainda ter posse do valor depois de usa-lo em uma função retornamos o valor da função
    let valor_string = String::from("ABCCARAIU");
    let retornando = retornando_dps_de_usar(valor_string);
    println!("{}", retornando);

    // Para poder usar o valor dessas variaveis sem perder sua posse usamos o reference "&"
    let inicial = String::from("exemplo");
    let batata = &inicial;
    println!("{} -> {}", incial, batata);

    // Um reference é imutavel de padrão ou seja ele não consegue alterar o valor da variavel inicial
    // Para isso é necessario especificar que a referencia é mutavel, assim como fazemos com uma variavel
    let mut japa = String::from("batata");
    let re = &mut abc;
    referecencia.push_str("abc");
    println!("referencia -> {}", re);
    println!("Original -> {}", japa);

}

fn tomar_posse_da_var(abc: String) -> () {
    println!("{}", abc);
    ()
}

fn retornando_dps_de_usar(cat: String) -> String {
    println!("Vou retornar {}", cat);
    return cat
}

