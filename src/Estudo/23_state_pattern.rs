// Iremos implementar o State pattern do rust
// basicamente o state pattern se resulme a um valor que possui estados interno 
// sendo que cada um desses estados internos possuem um comportamento 
// diferente para os metodos desse valor e um comportamento proprio
// de quando eles devem ou nao mudar o valor para outro estado

// Iremos deginir o state pattern em volta de triats objects 

// como exemplo iremos contruir uma programa que simula o funcionamento de segurança ao
// enviar postagens em uma rede social
// Para isso teremos uma struct post
// nesta estrutura voce invocara o metodo de criar_post() que criara e uma menssagem em um post
// após isso devemos um metodo para que a postagem seja revisada
// até que a revisão da menssagem seja feita, a postagem ficara indisponivel para o metodo ler_postagem()

struct Post {
    message: String,
    state: Option<Box<dyn State>>
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=--=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

// Inicalmente precisamos criar uma trait que ira conter os comportamentos padrões da estruct Post e
// gerenciar a mudança entre os estados de "Post".
// Além de ser ela quem vai definir quais são structs que representam estados, ela faz isso da seguinte forma:
// Todas as structs que a implementarem são consideradas States
trait State {
    // Definindo metodo padrão de pedido de revisao 
    // (temos que definir oque terá dentro de self tendo em vista que estamos definindo uma trait
    // exemplo: se precisarmos definir uma trait que tenho um metodo que possa modificar self, temos que definir
    // self: &mut Self)

    // Estamos especificando que o retorno do metodo revisão é um State pois ele retorna alguma struct que implementa State

    fn revisao(self: Box<Self>) -> Box<dyn State>;

    fn aprovar_revisao(self: Box<Self>) -> Box<dyn State>;
    
    fn ler_postagem(self: &Self, msg: &str);
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=--=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-


// Definindo struct que representao estado incial da postagem para quando o post acaba de ser criado
struct InitPostState();

impl State for InitPostState {
    fn revisao(self: Box<Self>) -> Box<dyn State> {
        Box::new(ReviewState {})
    }

    fn aprovar_revisao(self: Box<Self>) -> Box<dyn State> {
        println!("A postagem ainda não entrou para revisao");
        return self;
    }

    fn ler_postagem(self: &Self, msg: &str) {
        println!("");
    }

}


// Definindo struct que representao estado de review da postagem para quando o metodo de revisão é iniciada
struct ReviewState ();


impl State for ReviewState {
    fn revisao(self: Box<Self>) -> Box<dyn State> {
        println!("O estado de revisão já está ativo!");
        self
    }

    fn aprovar_revisao(self: Box<Self>) -> Box<dyn State> {
        Box::new(PublishedState {})
    }


    fn ler_postagem(self: &Self, msg: &str) {
        println!("");
    }
}

// Definindo struct que representao estado final da postagem para quando o review é aprovado e a postagem é publicada
struct PublishedState();

impl State for PublishedState {
    fn revisao(self: Box<Self>) -> Box<dyn State> {
        println!("A revisão já foi realizada!");
        self
    }

    fn aprovar_revisao(self: Box<Self>) -> Box<dyn State> {
        println!("A revisão já foi aprovada!");
        self
    }

    fn ler_postagem(self: &Self, msg: &str) {
        println!("{}", msg);
    }
}

// Não iremos precisamos definir uma struct que represeta o estado inicial da postagem, iremos usar a 
// propria struct "Post" que será o estado de quando um post acaba de ser iniciado


// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=--=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

impl Post {
    fn criar_post(msg: &str) -> Post {
        Post {  
            message: msg.to_string(),
            state: Some(Box::new(InitPostState()))
        }
    }


    fn revisao(&mut self) {
        // Aqui está a magica do state pattern
        // iremos capturar o valor do atributo state localizado dentro da struct Post usando o metodo take(),
        // ao fazer isso iremos capturar a posse do valor existente no atributo state (no caso uma struct 
        // que represente um estado)
        // este metodo captura a posse de um valor deixando no lugar onde ele capturou o valor None
        if let Some(estado) = self.state.take() {
            self.state = Some(estado.revisao());
        }
    }


    fn aprovar_revisao(&mut self) {
        if let Some(estado) = self.state.take() {
            self.state = Some(estado.aprovar_revisao());
        }
    }


    // Já que inicialmente toda Post inicializa com o InitState, toda vex que o ler_postagem
    // for chamado, devemos retornar ""
    fn ler_postagem(&mut self) {
        if let Some(estado) = self.state.take() {
            estado.ler_postagem(&self.message);
            self.state = Some(estado);
        }
    }
}

fn main() {
    let mut a = Post::criar_post("pato");
    a.revisao();
    a.aprovar_revisao();
    a.ler_postagem();
}