// estruct sao estruturas que funcionam como classe e sao capazes de armazenar 
// dados de diferentes types, comportamentos e funcionalidades, 
// pata definir uma struct primeiramente criamos um modelo do mesmo com o nome que queremos para nossa struct,
// Neste modelo definimos o nome atributos e seus types respectivamente
fn main() {
    struct Pessoa {
        nome: String,
        idade: u32,
        sexo: String,
    }

// Usando o modelo da struct criada
    let mut usuario = Pessoa {
        nome: String::from("Kiwi"),
        idade: 17,
        sexo: String::from("Masculino")
    };

// Se a variavel que estamos ultilizando o modelo de struct for mutavel 
// podemos alterar seus valores 
// Para mudar um valor expecifico da struct usamos "nome_da_var.atributo"
    usuario.nome = String::from("Sally");
    println!("{}", usuario.nome);

// para reciclar informações de outra variavel com structs e mudar apenas alguma informações
// podemos usar: "nome-da-var.." para explicitar que 
// as informações restantes do struct em questão reterão as informações já explcitadas em outra var com struct
    let user2 = Pessoa {
        nome: String::from("Kiwi"),
        ..usuario
    };


// Metodos são maneiras de se ter uma função ligada a uma classe ou a um struct
// Isso torna as coisas mais organizadas e seguras

    struct Retangulo {
        x: u32,
        y: u32
    }

// Estamos implementando area como metodo de Quadrado
// Para ter permissão de acessar os dados dentro do struct implementado colocamos como parametro &self
// self se refere ao proprio struct em questão e passa-lo como parametro faz com que possamos acessar o valores desse struct
    impl Retangulo {
        fn area(&self) -> u32{
            return self.x * self.y
        }
    }

    let abc = Retangulo {
        x: 12,
        y: 8
    };

    println!("{}", abc.area());

// Funções associadas são como metodos porem não resebem self para trabalhar com dados da Struct
// É comumente usado para retornar um novo formato da Struct em si
    struct Pato {
        nome: String,
        cor: String,
    }
    impl Pato  {
        fn new(name: String, color: String) -> Pato {
            return Pato {
                nome: name,
                cor: color,
            };
        }
    }

    let name = String::from("Jeff");
    let cor = String::from("Branco");
    let duck = Pato::new(name, cor);

}