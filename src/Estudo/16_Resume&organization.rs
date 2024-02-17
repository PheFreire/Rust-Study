// Ate agora vimos:
// 1- Funcoes, variaveis, types, escolpo, Tupla, Array, dados da pilha, dados heap, posse e referencias
// 2- Estruturas logicas (Match, if, loop, while, for) 
// 3- Srings(&str // Heap Strings), Vec
// 4- Structs, Enums, Option, Hash Maps
// 5- Tratamento de erros(Panic! & Result)
// 6- Generics (types // lifetimes // static references) e Traits 
// 7- IO
// 8- Iterator & closures
// Proximo: Smart Pointers

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-= //

// Notas ---> 

// Traits
// O conceito de Traits pode ser dado como uma serie de funcoes e limitacoes agrupadas que podem
// ser aplicadas a uma classes como um comportamento, 
// ou seja uma traits sao pacotes de comportamentos que uma classe ou type deve seguir   

// Structs
// Structs sao classes que recebem valores, funcoes, limitacoes e comportamentos
// Dentro de uma Struct podemos especificar atributos
// Struct Methods com o impl
// Structs Traits com o for
// Usar traits para definir comportamentos e metodos a um Generic types usando o where


// Generic types (gts)
// Usamos a estrutura generic "<>" para criar gts
// Gts são valores invocados como parametro de um method ou função,
// ao invocar gts expecificamos os comportamentos que
// o type que o substituirá deve ter obrigatorios
// eles são usados para expecificar que a func ou method em questão pode receber como parametros 
//  