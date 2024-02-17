fn main() {
    // Variaveis de exemplo
    let vec = vec![1, 2,3];
    let vec2 = vec![3, 2, 1];


    // Iterator é a estrutura que toda vez que é invocada retorna algum valor diferente
    // E possivel criar um obj iter de uma variavel complexa 
    // E toda vez que chamarmos esse iter_obj ele ira retornar um elemento diferente dessa variavel
    
    // O comando .iter() criara um objeto iter que existira em qnt a estrutura para qual ele foi chamado existir
    // ele invoca os valores como referencia, ou seja n toma pose dos valores 
    for e in vec.iter() {
        println!("{}", e);
    }


    // O comando into_inter() retorna um objeto iter que pode ser chamado em qualquer parte de seu escolpo
    // alem de que o into_iter() toma pose dos valores 
    let abc = vec.clone().into_iter();
    for e in abc {
        println!("{}", e);
    }


    // O comando zip cria um objeto iter duplo juntando dois outros objetos iter
    // Qnd esse obj duplo e chamado ele retorna uma tupla com o valor do primeiro e segundo objeto iter 
    let v1 = vec.into_iter();
    let v2 = vec2.into_iter();
    let iter = v1.zip(v2);
    for e in iter {println!("{:?}", e);}
    
    let abc = vec![1,5,7,8]; 


    // O comando .filter() cria uma condicao obrigatoria para manter os valores do iter
    // Se o resultado em seu parametro for False para o valor do obj_iter invocado, ele e retirado do iter
    // Sendo assim filtramos todos os resultados que derem o valor True 
    for e in abc.iter().filter(|x| x != &&1) {
        println!("{}",e);
    }  

    // O comando .map() realiza uma func anonima para cada valor retornado pelo objeto iter 
    // Lembrando que ainda e necessario chamar cada valor, o map nao aplica a funcao automaticamente
    // ou seja retorna os valores em formato de outro iterator
    for e in abc.iter().map(|x| x+1) {
        println!("{}", e)
    }
    
    // Realiza de maneira automatica uma funcao anonima para todos os valores de um obj iter
    let prin = abc.into_iter().for_each(|x: i32| {println!("{}", x + 1);});
    println!("{:?}", prin);


    // um iter e um trait que implementa diversos tipos de metodos
    // Por iterators serem apenas traits podemos criar nossos proprios iterators
    struct pato {
        num: i32,
    }
    
    impl pato {
        fn new(n: i32) -> pato {
            pato {num: n}
        }
    } 

    impl Iterator for pato {
        
    // O nome dado ao valor que e retornado do iterator e "Item"
    // Para configurar nosso proprio iterator precisamos definir qual o type do Item do nosso iterator  
        type Item = i32;

    // Irei montar o efeito do metodo next para o meu interator
    // Toda vez que ele for invocado ele ira retornar seu proximo valor ate o numero 5 
    // Obrigatoriamente o retorno de next e um option 
        fn next(&mut self) -> Option<Self::Item> {
            if self.num < 5 {
                self.num += 1;
                Some(self.num)
            }
            else {
                None
            }
        }
    }
    
    let mut p = pato::new(3);
    let capture = p.next();
    println!("{:?}", capture);
    let capture = p.next();
    println!("{:?}", capture);

}
