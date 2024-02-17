
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//              Oque são threads

    // Threads são objetos que fazem com que eventos possam funcionar simultaneamente
    // Durante um script os processos estão sendo executados dentro de apenas um nucleo do processador
    // programação multi-threading é repartir os processos de um script entre diferentes nucleos do 
    // processador 
    // programação multithreading deixa o programa muito otimizado, porem mais inseguro, "imprevisivel" e complexo
    // Para usar Threads dentro do Rust usamos o modulo nativo de threading no std


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//               Regras Threading

    // Um script sempre vai estar rodando em uma "main threading",
    // em um programa multithreading, quando a main threading for finalizada, 
    // todas as threading em processo serão interrompidas

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


use std::thread;
use std::time::Duration;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//                  Usando Threads

    fn _closure_in_threading() {

        // Para criar Threads de eventos anonimos (closures) chamamos 
        // o metodo spawn do objeto thread
        // Essa função retorna um closure em Threading
        // Pelos eventos dessa closure não estarem sendo processados na main thread
        // se quiser usar dados da main thread dentro dessa closure, será necessario
        // mover a posse do dado para dentro da closure usando a particula "move" 
        // Exemplo: thread::spaw(move || {println!("Duck!")});
        // (outro processo possivel, mais complexo e especifico seria usar SmartsP)


        // Ao invocar uma threading, podemos capturar o objeto correspondente a ela
        // dentro de uma variavel e assim manipular melhor seu funcionamento
        // ao fazer isso recebemos um objeto "handler"

        let hdl = thread::spawn(|| {
            let mut x = 0;
            loop {
                println!("{}", x);
                x = x+ 1;
    
                // A função thread::sleep() força uma parada na Thread e usa como parametro
                // o Type duration para explicitar a quantidade de tempo que a Thread ira ficar pausada 
                thread::sleep(Duration::from_millis(1));
            }
        });
    
        // Evento fora da threading
        for e in 0.. 10 {
            println!("Não sou uma Threading! {}", e)
    
        }

        // Como já foi dito, todos os eventos de um script estão sendo processados
        // em uma main thread, e qnd essa main thread tem seu processo finalizado
        // as outras threads (green threads) invocadas paralelamente
        // dentro desse mesmo script tem sua execução iterrompida,
        // porém, caso queiramos que a main thread espere o processo
        // de alguma green thread para continuar seu proprio processo, usamos o metodo join()
        
        // Usamos o metodo join em um objeto hanlder (objeto correspondente a uma green 
        // thread ativa no programa), o join(); irá retornar um Result<T> especificando
        // se o uso do join() foi ou não possivel
        hdl.join().unwrap();
        

    }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//          Passando dados entre threads

    // ~~~~~~~~~~~~~~~~~~~~~~~~
    //       Introdução

        // Basicamente podemos passar informação entre threads de maneira segura
        // atravez de canais de comunicação esses canais são chamados de chanels
        // para passar informações por um chanel, definimos um ponto de entrada 0de dados
        // e um ponto de saida 

        // Com chanels além de trocarmos informações entre threads, podemos por exemplo:
        // repartir um calculo entre diferentes threads e colocar um thread para juntar
        // todos os resultados finais da conta e retornar a resposta da operação

    // ~~~~~~~~~~~~~~~~~~~~~~~~
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~
    //    Importando modulos

        // Precisamos importar dentro do modulo de objetos da programação assincrona
        // o objeto correspondente ao nosso chanel: "mpsc" (multiple-producer single-consumer)
        use std::sync::mpsc;
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~
    
    // Pratica
    fn _send_recv_data() {

        // ~~~~~~~~~~~~~~~~~~~~~~~~
        //    Criando um Chanel 

            // O metodo channel retorna 2 objetos:
            // 1-O primeiro objeto é o objeto referente ao ponto de entrada de dados
            // 2-O segundo objeto é o referente ao ponto de saida, 
            // ou seja o objeto que ira receber o dado enviado de outra thread
            
            // Iremos criar o nosso chanel sempre dentro da da thread que ira receber um dado
            // da outra thread, pois dessa maneira o ponto de saida de dados estará
            // no escolpo correto
            let (enter_p, end_p) = mpsc::channel();
        
        // ~~~~~~~~~~~~~~~~~~~~~~~~


        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        //          Enviando dados por uma thread

            let _teste = thread::spawn(move || {
                // para enviar um dado usando o enter_p usamos o metodo send();
                // O metodo send retorna um result<T> 

                // Quando enviamos um valor entre Threads, obviamente também
                // enviamos sua posse
                let dado = String::from("Passando dados!");
                enter_p.send(dado).unwrap();
            });
        
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        //      Recebendo dados de outra thread

            // Podemos receber a resposta do enter point, de 3 maneiras possiveis
            // 1- recv();
            // 2- try_recv();
            // 3- iterators

            // ~~~~~~~~~~~~~~~~~~~~~~~~
            //      Metodo recv();

                // O primeiro metodo pausa a execução de onde ele estiver e só à continua
                // quando receber algum dado do enter point.
                // este metodo retorna um result<T> podendo conter a resposta do enter point)
                // ou um erro caso o chanel que estivermos tentando obter um dado esteja fechado
                end_p.recv().unwrap();
            
            // ~~~~~~~~~~~~~~~~~~~~~~~~


            // ~~~~~~~~~~~~~~~~~~~~~~~~
            //    Metodo try_recv();

                // O segundo metodo quando invocado, retorna instantaneamente um Result<T> 
                // caso o enter_p já tiver retornado alguma dado, iremos recebe-lo no Result
                // mas se o enter_p ainda não tiver retornado nada, receberemos um Err vazio
                // o try_recv(); é muito util quando ultilizado dentro de um loop 
                let _dat = loop {
                    match end_p.try_recv() {
                        Ok(dt) => {
                            println!("Recebi!");
                            break dt;
                        },
                        Err(_) => {continue;}
                    }
                };
            
            // ~~~~~~~~~~~~~~~~~~~~~~~~
    
            
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            //  End ponit como iterator

                // Também podemos receber a resposta do enter point usando
                // o end point como um iterator
                for retorno in end_p.iter() {
                    println!("{}", retorno);
                }
            
            // ~~~~~~~~~~~~~~~~~~~~~~~~
                
        
        // ~~~~~~~~~~~~~~~~~~~~~~~~
    
        
    }

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//      Recebendo dados entre diversas threads

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Recebendo dados de diversas threads em um end point

        fn many_threads() {
            // Criando chanel
            let (ep, endp) = mpsc::channel();


            // Para receber dados de mais de uma thread em um end point
            // precisamos criar um clone do objeto enterpoint
            // para isso usamos o metodo clone();
            let copia = ep.clone();

            // Enviado dados pela thread 1
            thread::spawn(move || {
                let texto = String::from("Textinho Thread 1");
                ep.send(texto).unwrap();
            });

            
            // Enviado dados pela thread 2 usando o clone 
            thread::spawn(move || {
                let texto = String::from("Textinho Thread 2");
                copia.send(texto).unwrap();
            });

            for e in endp.iter() {
                println!("{}", e);
            }
        }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//                    Mutex

    // Para que uma variavel possa ser acessada dentro de uma Thread 
    // Sem que sua posse seja passada para dentro da thread
    // podemos usar o smartP Mutex
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //                 Funcionamento

        // Como já vimos, SmartP podem armazenar informações extras dentro de sí
        // como o Rc<T> que guarda a quantidade de elementos que possuem sua posse.
        // No caso do Mutex além do elemento que ele armazena,
        // ele contem o atributo "Lock" este é atualizado constantemente com
        // a informação de qual thread está o acessando e bloqueia o acesso
        // de todas as outras, assim mantendo as regras de ownership e borrowing
        // e mantendo a pilha segura

    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //              Importando Mutex
    
        use std::sync::Mutex;
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //          Criando e usando Mutex

        fn Basic() {
            // Ao iniciarmos o nosso Mutex, ele vem com o atributo lock travado  
            let mut data = Mutex::new(5);

            // Para destravar o Mutex "data" usamos o metodo "lock();"
            // este metodo muda os status do atributo lock e altera o type
            // da variavel data de: "Mutex" para "MutexGuard"
            // está mudança de type indica que agora podemos acessar o valor 
            // dentro de "data".
            // O MutexGuard possui um Drop trait, 
            // este trava novamente o atributo lock quando o MutexGuard sai do escolpo
            // assim não precisamos nos preocupar em destrava-lo

            {
                // o metodo lock(); retorna um result indicando se a operação deu certo
                let unlock_data  = data.lock().unwrap();
                println!("{}", *unlock_data);
            }

        }

        fn real_use() {
            // Para o exemplo a seguir iremos usar o SmartP Arc<T>
            // este SmartP funciona exatamente igual o Rc<T>
            // sua unica diferença é que ele ao contrario do Rc<T>
            // pode gerenciar multiplas posses de um elemento entre Threads 
            use std::sync::Arc;

            let dt = Arc::new(Mutex::new(0));
            let mut threads = vec![];

            for _ in 0..10 {
                let clon = Arc::clone(&dt);
                let t = thread::spawn(move || {
                    let mut usar = clon.lock().unwrap();
                    *usar += 1
                });
                threads.push(t);
            }

            for a in threads {
                a.join().unwrap();
            }

            println!("{}", *dt.clone().lock().unwrap());
        }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Criando comportamento multi-Thread para os nossos SmartP

    // Para manipular o comportamento multi-thread dos nossos proprios SmartP
    // usamos 2 traits:
    // -"Send" ---> quase todos os types e SmartP do rust implementam a trait "Send",
    // os type que implementa o "send" possui a propriedade de poder ter sua posse passada entre threads
    // esta trait também nos permite gerenciar como ira ocorrer essa passagem de posse entre Theads
    // O smartP Rc<t> não implementa o send pois se sua posse for passada para outra thread, 
    // o contador de mutiplas posses não saberá como se comportar e perderá desempenho e segurança
    // para esse caso usamos o SmartP Arc<T> q serve exatamente para esse proposito

    // -"Sync" Os types que implementam a trait Sync estão indicando que é seguro que multiplas
    // threads possam se referenciar ao seu valor sem possuir sua posse. ou seja o type "T" implementa
    // Sync caso "&T" implemente "Send"

    // Para implementar p Sync e o Send precisaremos usar unsafe rust
    // e por enquanto essa não é nossa praia, ent não vamos cobrir esta parte com exemplo pratico

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn main() {
    many_threads();
    // closure_in_threading();
}