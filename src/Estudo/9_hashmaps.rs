fn main() {
    // um hash map (HM) é o nome generico dado a dicionarios, como o dict do python
    // uma maneira de guardar valores usando keys
    // É a estrutura de dados mais veloz que armazena seus dados na pilha heap
    // coomo na String, para usar seus metodos é necessario importar seu modulo na std
    use std::collections::HashMap;
    use std::collections::String;

    // Para criar um HashMap vazio, usamos o metodo ::new();
    let mut abc = HashMap::new();

    // Para adicionar valores a um HashMap usamos o metodo .insert(key, value);
    abc.insert(String::from("ip"), String::from("192.168.15.119"));

    // Também podemos criar HM usando a func collect();
    // Collect() é uma func q pd ser usada para transformar iterators em coleções de dados
    // Como exemplo eu tenho 2 Vec<T> um com as keys que quero para meu HM 
    // e outro de values para essas keys respectivamente 
    let keys  = vec![String::from("K_one"), String::from("K_two")];
    let values = vec![1, 22];

    // Para usar o collect(); primeiro crio um objeto iter cm o metodo zip para "fundir" o retorno dos 2 Vecs,
    // de primeira o collect detecta o tipo de variavel heap para onde ele ira retornar seus resultados,
    // após detectar o type HM o collect() se prepara para receber os dados do objeto iter com os vecs fundidos
    // O collect cria um HM usando o primeiro retorno do obj iter como key e o segundo retorno do obj inter cm value da key anterior
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    println!("{:?}", scores);

    // Para invocar valores em um HM usamos a func get(key), que como já sabemos retorna o enum Some<T>, 
    // podendo retornar o valor correspondente a keys invocada no get (caso a key exista) e no caso da key não existir retorna None
    match scores.get("K_one") {
        Some(val) => {println!("{}", val);},
        Some(None) => {},
    
    }

    // Podemos retornar valores de um HM usando for, 
    // isso fará o HM retornar seus valores da mesma maneira que o metodo .enumerate() em um Vec<T>, 
    // retornando o endereço do valor value o value
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Substituindo valores em um HM
    // Para sobrescrever os values já existentes em uma key apenas usamos insert() na key existente com o novo value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Para inserir um value dentro de uma key apenas se ela nao possuir values usamos
    // HM.entry(key).or_insert(value), o entry vai conferir a existencia de um value para a key analisada
    // Caso a key nao possua values o .or_insert(value) vai inserir um value para a key em questao
    scores.entry(String::from("Yellow")).or_insert(0);

    // O HM.entry().or_insert() tambem retorna uma referencia mutavel da key inserida
    // sendo assim podemos nos referir a uma key e ao valor existente dentro da mesma e altera-lo
    // Exemplo vou contar qnts palavras existem em uma frase e armazenar qnts vezes ela foi usada
    // Guardarei a palabra e qnts vezes foi ultilizada dentro do dicionario como key e value respectivamente
    let mut contagem = HashMap::new();
    let abc = "pato pata pato pata ganso gay pata gat batata yo yoty";
    for e in abc.split_whitespace() {
        let word = contagem.entry(e).or_insert(0);
        // Ja que entry() retorna uma reference para q o valor possa ser alterado tomamos posse do mesmo usando "*"
        *word += 1;
    }

}