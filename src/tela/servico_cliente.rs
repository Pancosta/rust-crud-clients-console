use crate::models::cliente::Cliente;
use super::{ler::*, operacoes_basicas::*};

pub fn incluir_cliente(clientes: &mut Vec<Cliente>){

    limpar_tela();

    let mut cliente: Cliente = Cliente::default();
    cliente.id = clientes.len() + 1;
    digitar_dados_do_cliente(&mut cliente);



    clientes.push(cliente);

    limpar_tela();
    println!("Cliente cadastrado com sucesso!");

    esperar(1);

}

pub fn listar_clientes(clientes: &mut Vec<Cliente>){
    limpar_tela();
    if checar_clientes(clientes){
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
pub fn alterar_cliente(clientes: &mut Vec<Cliente>){
    limpar_tela();
    if checar_clientes(clientes){
        return;
    }

    let id = captura_id();
    if let Some((indice,cliente)) = buscar_cliente_id(clientes,id){
        linha();
        println!("Alterando o cliente...");
        linha();
        mostrar_cliente(cliente);
        linha();
        digitar_dados_do_cliente(&mut clientes[indice]);
        limpar_tela();
        println!("Alteração concluida!");
    }else{
        limpar_tela();
        println!("Cliente não encontrado!");
    }
    
    esperar(1);

}
pub fn excluir_cliente(clientes: &mut Vec<Cliente>){
    limpar_tela();
    if checar_clientes(clientes){
        return;
    }

    let id = captura_id();
    if let Some((indice,cliente)) = buscar_cliente_id(clientes,id){
        linha();
        println!("Confirma a exclusão do cliente abaixo?");
        linha();
        mostrar_cliente(cliente);
        linha();
        println!("Digite 's' para confirmar a exclusão:");
        let opcao = ler_dados();
        if opcao == "s"{
            clientes.remove(indice);
            limpar_tela();
            println!("Cliente excluído com sucesso!");
            esperar(1);

        }else{
            limpar_tela();
            println!("Exclusão Cancelada!");
            esperar(1);
        }
        
        
    }else{
        limpar_tela();
        println!("Cliente não encontrado!");
    }
    
    esperar(1);

}


fn captura_id() -> usize {
    limpar_tela();
    println!("Digite o ID do cliente:");
    ler_dados_int()
}

fn buscar_cliente_id(clientes: &Vec <Cliente>,id:usize) -> Option<(usize, &Cliente)>{
    clientes.iter().enumerate().find(|(_,cliente)| cliente.id == id)
}

fn digitar_dados_do_cliente(cliente: &mut Cliente){
    println!("Digite o Nome:");
    cliente.nome = ler_dados();
    println!("Digite o Cpf:");
    cliente.cpf = ler_dados();
    println!("Digite o Endereço:");
    cliente.endereco = ler_dados();
}

fn mostrar_cliente(cliente: &Cliente) {
    println!("\
        ID: {}\n\
        Nome: {}\n\
        Cpf: {}\n\
        Endereço: {}\
",cliente.id,cliente.nome,cliente.cpf,cliente.endereco);
}


fn checar_clientes(clientes: &mut Vec<Cliente>)-> bool{
    if clientes.len() == 0{
        println!("Não existem clientes cadastrados!");
        esperar(1);
        return true;
    }else{
        return false;
    }
}


