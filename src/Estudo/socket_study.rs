use std::net;
use std::io::{Read, Write};
use std::thread::spawn;
use std::sync::{Arc, mpsc::channel};


fn main() {
    // 192.168.15.135
    let mut network = Server::new([192, 168, 15, 135]);
    network.init_server(true);
    
    
}


struct Server {
    
    socket: net::SocketAddrV4,

    // Objeto cliente ativo
    clients: Vec<(String, net::TcpStream)>,

    // Quantidade de clientes ativos
    client_num: usize,

    // (Room Name, password, max qnt)
    rooms: Vec<(String, String, usize)>,

}





impl Server {

    /// Metodo de inicialização do servidor
    pub fn new(ip: [u8;4]) -> Server {
        // Para ultilziar um ip precisamos criar um objeto ip 
        // Temos 3 estruturas ip que correspondem a objetos ip, sendo elas
        // 2 structs e uma enum 
        
        // A Struct: std::net::Ipv4Addr -> Se refere a um ipv4
        // A Struct: std::net::Ipv6Addr -> se refere a um ipv6
        // A Enum: std::net::IpAddr -> pode conter dentro de si ou Ipv4Addr ou um Ipv6Addr

        // Para criar um Objeto ip colocamos o nome da estrutura ip e chamamos o metodo "new();"
        // colocando como parametro u8 correspondentes ao endereço ip 

        let server_ip = net::Ipv4Addr::new(ip[0],ip[1], ip[2], ip[3]);
        let data = Server {
            // Temos 2 structs que podem corresponder a um objeto socket sendo elas:
            // std::net::SocketAddrV4 -> corresponde a um objeto socket que ultiliza o protocolo ipv4
            // std::net::SocketAddrV6 -> corresponde a um objeto socket que ultiliza o protocolo ipv6

            // Para criar um objeto socket chamamos o metodo "::new(objeto_ip, porta de rede);"
            // para a struct SocketAddrV4 ou SocketAddrV6
            socket: net::SocketAddrV4::new(server_ip , 52577),
            clients: vec![], 
            client_num: 0,
            rooms: vec![]
        };
        return data;
    }

    
    /// Inicializa o processo de recebimento de clientes do servidor
    pub fn init_server(&mut self, err: bool) {

        // Um TcpListener é um objeto que permite com que um objeto socket
        // possa efetuar conexões tcp com outros sockets
        // Para criar um TcpListener nos precisamos hospeda-lo em um objeto socket para isso
        // chamamos o metodo net::TcpListener::bind(objeto socket); 
        // como parametro colocamos o socket no qual queremos hospedar nosso TcpListener
    
        let listen = net::TcpListener::bind(self.socket).unwrap_or_else(|_| {
            error("Problema ao receber clientes no servidor!", err);
            panic!();
        });

        
        let (enter, end) = channel();
        let listen = Arc::new(listen);
        let entrada = enter.clone();
        
        spawn(move || {
            loop {
                // Um TcpListener tem a funcionalidade basica de receber 
                // clientes (objetos sockets) atravez da conexão tcp 
                // e para fazer isso temos dois metodos:
                // O metodo accept(); -> aceita uma conexão de cliente cada vez que é invocado
                // O metodo incoming() -> retorna um iterator() que fica em loop a espera da conexão de clientes
                // e toda vez que um cliente é conectado ela retorna o objeto socket deste cliente 
                for e in listen.incoming() {
                    match e {
                        Ok(client) => {
                            entrada.send(client).unwrap_or_else(|_| {
                                error("Err: erro ao comunicar a entrada do cliente com o servidor", err);
                            });},
                        Err(_) => {}
                    }
                }
            }
        });

        loop {
            for e in end.iter() {
                println!("Cadastrando cliente!");
                self.log(e, err);
                self.show_server_atributes(true);
                self._test_conection(false);
         }
        }
    }


    /// Loga um cliente dentro da do servidor
    fn log(&mut self, socket: net::TcpStream, err: bool) -> bool {
        
        // Recebendo nome do cliente
        let name = match Server::recv_data(&mut socket.try_clone().unwrap(), 1024, err) {
            Ok(dt) => {dt},
            Err(_) => {return false;}
        };

        // Conferindo se já não foi cadastrado um cliente com o mesmo nome da variavel "name" 
        let conffer = |name| {
            for e in self.clients.iter().map(|(x, _)| x == name) {
                if e {
                    return false;
                }
            }
            return true;
        };

        if conffer(&name) {
            // Cadastrando cliente
            self.clients.push((name.clone(), socket));
            return true;
        }
        else {
            error("Este nome de usuario já está cadastrado no servidor!", err);
            return false;
        }
    }


    /// Printa as informações do servidor
    pub fn show_server_atributes(&self, adv: bool) {
        println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
        println!("Há {} usuarios conectados no servidor:", self.client_num);
        println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
        if adv {
            println!("Lista de clientes conectados no servidor:");
            println!("{:?}", self.clients);
            println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
        }
        println!("Há {} salas ativas:", self.rooms.len());
        println!("{:?}", self.rooms);
        println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    }


    /// Remove cliente do servidor
    pub fn _remove(&mut self, name: &'static str, err: bool) -> bool {

        // Retorna posição do cliente que será deletado
        match self._find_client(name, true) {
            // Caso o cliente seja encontrado suas indexes em "clients" sera retornada
            Some(client_index) => { 
                self.clients.remove(client_index);
                self.client_num = self.clients.len();
                return true;
            },

            None => {
                error("Erro ao tentar remover cliente do server: cliente não encontrado!", err);
                return false;
            }
        }
    }


    /// Preocura e retorna a index correspondentes a um cliente dentro das listas do servidor  
    fn _find_client(&self, name: &'static str, err: bool) -> Option<usize> {
        
        let lista = self.clients.iter().map(|e| e.0 == name);
        
        for (bl, e) in lista.zip(self.clients.iter().enumerate()) {
            if bl {
                return Some(e.0);
            }
        }

        error("Cliente não encontrado!", err);
        return None;
      
    }


    /// Recebe dados de clientes
    fn recv_data(socket: &mut net::TcpStream, buffer: usize ,err: bool) -> Result<String, &'static str> {
        let mut getter: Vec<u8> = vec![];

        for _ in 1..=buffer {
            getter.push(0);
        }

        loop {
            let size = socket.read(&mut getter).unwrap_or_else(|_| {
                error("Err: Tamanho de dado recebido não suportado pelo servidor!", err);
                0
            });
            
            if size != 0 {
                if size > buffer {
                    error("Err: Tamanho de dado recebido não suportado pelo servidor!", err);
                    return Err("Err: Tamanho de dado recebido não suportado pelo servidor!");
                } 
                else {
                    match String::from_utf8(getter[..size].to_vec()) {
                        Ok(dt) => {return Ok(dt);},
                        Err(_) => {
                            error("Err: Erro ao receber dados do cliente!", err);
                            return Err("Err: Erro ao receber dados do cliente!");
                        }
                    }
                }
            } else {
                error("Err: Não há dados para a serem lidos!", err);
                return Err("Err: Não há dados para a serem lidos!");
            }
        }
    }


    fn _test_conection(&mut self, err: bool) {
        let mut to_remove: Option<usize> = None;
        loop {
            let data = &self.clients;

            match to_remove {
                Some(cli_err) => {
                    self.clients.remove(cli_err);
                    self.client_num = self.clients.len();
                    to_remove = None;
                    println!("{:?}", self.clients);
                },
                None => {
                    if data.len() > 0 {
                        let len = data.len();
                        for i in (0..len).into_iter() {
                            if !!! self.send(i, "$", err) {
                                to_remove = Some(i);
                                break;
                            }
                        }
                    }
                }
            }              
        }
        
    }


    fn send(&self, idx: usize, txt: &str, err: bool) -> bool {
        let snd = format!("{}", txt);
        
        let mut cli = &self.clients[idx].1;

        let erro = cli.write(snd.as_bytes()).unwrap_or_else(|_| {
            error("Err: erro ao registrar dados para enviar o servidor!",err);
            0
        });

        match cli.flush() {
            Ok(_) => {
                if erro != 0 {return true;}
                else {return false;}
            }
            Err(_) => {
                error("Err: erro ao enviar dados para o servidor!",err);
                return false;
            }
        }
    }
}


fn _inn<T: PartialEq> (list: &Vec<T>, data: T) -> Option<usize> {
    for (i, e) in list.iter().enumerate() {
        if e == &data {
            return Some(i);
        }
    }
    return None;
}



fn error(err: &'static str, condition: bool) {
    if condition {
        println!("{}", err);
        std::process::exit(1);
    }
}