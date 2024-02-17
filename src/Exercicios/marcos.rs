use std::collections::HashMap;
use std::io;
use std::fs;

struct Item {
    qnt: i64,
    val: f64
}

impl Item {
    fn add(&self, q: i64) -> Item{
        return Item {qnt: self.qnt + q, val: self.val}; 
    }

    fn sub(&self, q: i64) -> Item{
        return Item {qnt: self.qnt - q, val: self.val}; 
    }
    
    fn _change_value(&self, new: f64) -> Item {
        return Item {qnt: self.qnt, val: new}; 
    }
}

enum Possible {
    Float(f64),
    Ss(String),
    Int(i64)
}

fn _get_io_values(typ: &str) -> Possible {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Error");
    let data = data.trim();
    if typ == "f" {
        let data: f64 = data.parse().expect("Value Error");
        return Possible::Float(data);
    }
    else if typ == "s" {
        return Possible::Ss(data.to_string());
    }
    else if typ == "i" {
        let data: i64 = data.parse().expect("Value Error");
        return Possible::Int(data);
    }
    else {
        panic!("Erro");
    }
}

fn capture_data() -> HashMap<String, Item> {
    let data = loop { 
        match fs::read_to_string("data.txt") {
            Ok(d) => {break d;},
            Err(_) => {fs::File::create("data.txt");continue;}
        };
    };
    let dt: Vec<&str> = data.split("\n").collect();
    let mut finall: HashMap<String, Item> = HashMap::new();
    
    for e in dt.iter() {
        let dd: Vec<&str> = e.split("/").collect();
        if dd.len() == 3 {
            let qnt = dd[1].trim().parse().expect("erro");
            if qnt > 0 {
                let inv = Item{
                    qnt: qnt, 
                    val: dd[2].trim().parse().expect("erro")
                };
                finall.entry(dd[0].to_string()).or_insert(inv);
            }
        }       
    }
    return finall;
}

fn main() {
    loop {
        let data = capture_data();
        println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
        println!("Add item (1) // Delete item (2) // Sub item (3) // Check Inventory (4) // Change Item Price (5) // Exit (6)");
        let qrer = match _get_io_values("s") {
            Possible::Float(_) => {panic!("nop");},
            Possible::Ss(n) => n,
            Possible::Int(_) => {panic!("nop");}
        };

            if qrer == "1" {create_or_change_val(data, false);}
        
            else if qrer == "2" {delete_item(data);}
        
            else if qrer == "3" {create_or_change_val(data, true);}

            else if qrer == "4" {
                for (e, i) in  data.iter() {
                    println!("item {}, qnt {}, val {}", e, i.qnt, i.val);
                }
            }

            else if qrer == "5" {
                chage_item_price(data);
            }

            else if qrer == "6" {
                break;
            }

            else {
                println!("Request error, try again...");
            }
    }

}


fn create_or_change_val(mut data: HashMap<String, Item>, sub: bool) {
    loop {
        if sub == false {println!("Coloque o nome do item que você quer criar ou adicionar exemplares -> ");}
        else {println!("Coloque o nome do item que você quer retirar exemplares -> ");}
        let name = match _get_io_values("s") {
            Possible::Float(_) => {println!("Value Error");continue;},
            Possible::Ss(n) => n,
            Possible::Int(_) => {println!("Value Error");continue;}
        };
        
        match &data.get(&name) {
            Some(ppp) => {
                if sub == false {println!("Existem {} exemplares!, quando você deseja adicionar? -> ", ppp.qnt);}
                else {println!("Existem {} exemplares!, quando você deseja retirar? -> ", ppp.qnt);}
                let number = match _get_io_values("i") {
                    Possible::Int(i) => i,
                    Possible::Float(_) => {println!("Value Error");continue;},
                    Possible::Ss(_) => {println!("Value Error");continue;}
                };
                if sub == false {data.insert(name, ppp.add(number));}
                else {data.insert(name, ppp.sub(number));}
                save_data(&data);
                break;
            },
            None => {
                if sub == true {println!("O item {} não existe", &name);continue;}
                else {
                    println!("O item {} não existe, gostaria de adiciona-lo? (s) / (n) -> ", &name);
                    match _get_io_values("s") {
                        Possible::Int(_) => {println!("Value Error");continue;},
                        Possible::Float(_) => {println!("Value Error");continue;},
                        Possible::Ss(n) => {
                            if n == "s" {
                                //
                                let item_add = Item {
                                    qnt: {   
                                            println!("Quantidade de {} -> ", &name);
                                            match _get_io_values("i") {
                                            Possible::Float(_) => {println!("Value Error");continue;},
                                            Possible::Ss(_) => {println!("Value Error");continue;},
                                            Possible::Int(e) => e
                                        }
                                    },
                                    val: 
                                        {
                                            println!("Preço do produtor {} (coloque ponto para separar o real dos centavos) -> ", &name);
                                            match _get_io_values("f") {
                                            Possible::Float(e) => e,
                                            Possible::Ss(_) => {println!("Value Error");continue;},
                                            Possible::Int(_) => {println!("Value Error");continue;}
                                        }
                                    }
                                };
                                data.insert(name.clone().to_string(), item_add);
                                save_data(&data);
                                break;
                            }   else {break;}
                        }  
                    }      
                }
            }    
        }
    }
}


fn save_data(data: &HashMap<String, Item>) {
    let mut finall = String::new(); 
    for (k,v) in data.iter() {
        let dado = format!("{}/{}/{}\n",k,v.qnt,v.val);
        finall.push_str(&dado);
    }
    match fs::write("data.txt", finall) {
        Ok(_) => println!("Data Updated!"),
        Err(_) => panic!("Data not updated!")
    }
}


fn delete_item(mut data: HashMap<String, Item>) {
    loop {
        println!("Qual item você gostaria de deletar?");
        println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
        for e in data.keys() {
            println!("{}", e);
            println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
        }
        println!("---> ");
        match _get_io_values("s") {
            Possible::Float(_) => {println!("Value Error");continue;},
            Possible::Int(_) => {println!("Value Error");continue;},
            Possible::Ss(n) => 
            {
                match data.remove(&n) {
                    None => {println!("Item {} não encontrado!", {n});continue;},
                    Some(_) => {println!("Deletado!");save_data(&data);break}
                }
            }
        }
    }
    
}

fn chage_item_price(mut data: HashMap<String, Item>) {
    loop {
        println!("Qual item que você gostaria de mudar o preço -> ");
        match _get_io_values("s") {
            Possible::Int(_) => {panic!("Value Erro");},
            Possible::Float(_) => {panic!("Value Erro");},
            Possible::Ss(e) => {
                match data.get(&e) {
                    None => {println!("Item {} não encontrado...", &e);break;},
                    Some(n) => {
                        println!("O valor atual de {} é: {}\nPor favor insira o novo valor ->  ", &e, n.val);
                        match _get_io_values("f") {
                            Possible::Int(_) => {panic!("Value Erro");},
                            Possible::Ss(_) => {panic!("Value Erro");},
                            Possible::Float(fl) => {
                                data.insert(e, n._change_value(fl));
                                save_data(&data);
                                break;
                            },
                        }
                    }
                }
            },
        }
    }
}
