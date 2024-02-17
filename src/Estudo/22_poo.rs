// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//              POO

    // A programação orientada a objetos é baseada em 3 pontos:

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //            Objetos

        // O conceito de objetos é criar um elemento com 
        // dados e comportamentos proprios, e dentro do rust 
        // nos podemos interpretar isso como structs, enums e impl

        struct _Class {
            data: Vec<usize>
        }

        impl _Class {
            fn _new(dt: usize) -> _Class {
                _Class {data: vec![dt]}
            }
        }

        fn _classe() {
            let _d = _Class::_new(5);
        }
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //         Encapsulamento

        // Encapsulamento é o conceito de deixarmo inacessivel para o ususario o 
        // uso de metodos e alterações de valores dentro de classes
        // no rust podemos expecificar oque será publico ou privado
        
        struct _Ex {
            pub lista: Vec<i32>,
            tamanho: usize
        }

        impl _Ex {
            pub fn new(num: &Vec<i32>) -> _Ex {
                return _Ex{lista: num.to_vec(), tamanho: num.len()};
            }

            pub fn add(&mut self, num: i32) {
                self.lista.push(num);
                self.update_tamanho();
            }

            fn update_tamanho(&mut self) {
                self.tamanho = self.lista.len();
            }

            fn delete(&mut self, idx: usize) {
                self.lista.remove(idx);
                self.update_tamanho();
            } 
        }
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //             Herança

        // Herança seria poder herdar metodos e elementos 
        // de uma classe para outra
        // No rust não podemos faazer isso, o mais proximo que nos chegamos
        // de conseguir implementar a herança é usando traits onde nos podemos
        // expecificar comportamentos (metodos) e importar esse metodos para diferentes structs
        // porem não podemos importar elementos como atributos e afins
        // Ex: 

        fn heran_1() {

            trait Aleatoria {
                fn new_aleatoria<T>(dt: T) -> T;

                fn return_my_self(self) -> Self;
            }

            impl Aleatoria for _Ex {
                fn new_aleatoria<T>(dt: T) -> T {
                    return dt;
                }

                fn return_my_self(self) -> Self {
                    return Self {lista: vec![10], tamanho: 0}
                }

            }
        }

        fn trait_obj() {
            // Porem dentro do Rust podemos criar um conjunto de dados
            // no qual nos não expecificamos os types dos elementos que ele pode armazenar,
            // mas sim uma trait obrigatoria no qual todos os elementos dentro desse conjunto
            // de dados tem que obrigatoriamente implementar
            
            // Dessa maneira podemos criar uma struct que contem um conjunto de dados
            // que tem que obrigatoriamente implementar certos comportamentos e o nosso
            // disso é: Trait Objects

            // Para criar um Trait object precisamos epecificar um pointer (Reference ou SmartP)
            // e dentro desse pointer expecificar a trait obrigatoria que o type que será colocado deve,
            // implementar

            // Você deve estar se perguntando o porque de precisarmos de um pointer para usar um trait Object
            // o motivo disso é que durante o tempo de compilação, o compilador vai descobrir todos os types
            // dentro do script que podem implementar a trait expecificada e irá apontar para aqueles types
            // expecificando assim que aquele trait Object em questão pode ser substituido por qualquer
            // um dos types apontados

            pub trait Duck {
                fn quack(&self, dt: String) -> String;
            }

            struct _P {
                dt: String
            }

            impl Duck for _P {
                fn quack(&self, dt: String) -> String {
                    return format!("O pato diz {}", dt);
                }
            }
            
            // Quando o compilador não sabe os types que implementam a trait do trait object em questão
            // para expecificar para o compilador que o trait object pode ser descoberto em tempo de execução
            // precisamos usar a keyword "dyn" antes de colocar a trait no trait object
            // isso diferentemente do processo de descoberta dos types de generics types por exemplo
            // que descobre os types em tempo de compilação e mantem o programa de maneira estatica
            // o dyn expecifica que o programa deve descobrir ps types de maneira dinamica

            // Existem duas regras para se criar um trait object com o safe rust
            // -não se pode ter generics types dentro da trait
            // -não se pode retornar Self dentro de nenhum metodo da trait

            
            struct _Implementandoheranca {
                conjunto: Vec<Box<dyn Duck>>
            }



            impl _Implementandoheranca {
                fn loopp(&self) {
                    for (e, b) in self.conjunto.iter().zip(0..self.conjunto.len()) {
                        e.quack(format!("{}",b));
                    }
                }
            }
        }
    
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


fn main() {
    
}