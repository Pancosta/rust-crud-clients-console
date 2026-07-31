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