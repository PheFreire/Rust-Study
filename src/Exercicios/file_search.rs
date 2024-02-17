// "std::env" modulo que nos dá acesso a funções de processo de ambiente, 
// tais como variaveis de ambiente e outros
use std::env;

struct Configs {
    txt: String,
    search: String,
}


impl Configs {
    
    fn new() -> Result<Configs, &'static str> {
        // std::env::args retorna um iterator com todos os parametros de terminal
        // encontrados quando o script foi inicializado
        // usaremos collect() para transformar o iterator em um conjunto de dados
        let pp: Vec<String> = env::args().collect();
        if pp.len() < 3 {
            return Err("Não há argumentos suficientes para realizar essa operação!");
        }
        return Ok(Configs {txt: pp[1].clone(), search: pp[2].clone()});
    }

    fn get_text(&self) -> Result<Vec<String>, &'static str> {
        // Preparando variavel que receberá em formatdo de String os dados do arquivo q queremos abrir
        let data = std::fs::read_to_string(&self.txt);
        match data {
            Ok(a) => {
                let lin: Vec<String> = a.split("\n").map(|x| x.to_string()).collect();
                return Ok(lin);},
            Err(_) => {return Err("Erro de leitura!");}
        }
    }


    fn find(&self, txt: &Vec<String>) -> Result<Vec<usize>, String> {
        let mut lines: Vec<usize> = vec![];
        for (i, e) in txt.iter().enumerate() {
            if e == &self.search {
                lines.push(i + 1);
            }
            
        }
        if lines.len() > 0 { 
            return Ok(lines);
        }
        else {
            return Err(format!("{}, não foi encontrada dentro do arquivo!", self.search));
        }
    }

}


fn analysis<T, O>(data: Result<T, O>) -> T 
    where O: std::fmt::Display {
    // unwrap_or_else, retorna o valor de Ok caso o valor do result seja um Ok
    // e se o valor do result for Err ele chama uma função 
    // anonima passando o parametro de Err como parametro dessa função anonima
    let ok = data.unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });
    ok
}


fn main()  {
    println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=\n");
    
    let data = analysis(Configs::new());
    let text = analysis(data.get_text());
    let fr = analysis(data.find(&text));

    println!("A palavra {} foi encontrada {} vezes,\ne pode ser encontrada nas linhas {:?} ", data.search, fr.len(), fr);
}