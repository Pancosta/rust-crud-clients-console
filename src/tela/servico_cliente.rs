use crate::models::cliente::Cliente;
use super::{ler::ler_dados, operacoes_basicas::*};

pub fn incluir_cliente(clientes: &mut Vec<Cliente>){

    limpar_tela();

    let mut cliente: Cliente = Cliente::default();

    cliente.id = clientes.len() + 1;
    println!("Digite o Nome:");
    cliente.nome = ler_dados();
    println!("Digite o Cpf:");
    cliente.cpf = ler_dados();
    println!("Digite o Endereço:");
    cliente.endereco = ler_dados();



    clientes.push(cliente);

    limpar_tela();
    println!("Cliente cadastrado com sucesso!");

    esperar(1);

}

pub fn listar_clientes(clientes: &mut Vec<Cliente>){
    limpar_tela();

    if clientes.len() == 0{
        println!("Não existem clientes cadastrados!");
        esperar(1);
        return;
    }


    linha();

    for cliente in clientes{
        mostrar_cliente(cliente);
        linha();
    }

    println!("Digite enter para continuar...");
    ler_dados();

}

fn mostrar_cliente(cliente: &mut Cliente) {
    println!("\
        ID: {}\n\
        Nome: {}\n\
        Cpf: {}\n\
        Endereço: {}\
",cliente.id,cliente.nome,cliente.cpf,cliente.endereco);
}