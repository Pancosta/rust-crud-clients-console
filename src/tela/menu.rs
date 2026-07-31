use crate::tela:: ler::*;
use crate::tela:: operacoes_basicas::*;
use crate::models::cliente::Cliente;
use crate::tela::servico_cliente::*;
pub fn mostrar_menu(clientes: &mut Vec<Cliente>){
    limpar_tela();
    loop{
        limpar_tela();

        println!("\
            ============ Menu ===========\n\n\
            Escolha uma das opções abaixo:\n\n\
            1 - Cadastrar cliente\n\
            2 - Alterar cliente\n\
            3 - Excluir cliente\n\
            4 - Listar clientes\n\
            0 - Sair do programa\n\
        ");

    let opcao = ler_dados_int();
   
    match opcao{
        1 => incluir_cliente(clientes),
        2 => println!("Opção 2"),
        3 => println!("Opção 3"),
        4 => println!("Opção 4"),
        0 => {
            println!("Finalizando ...");
            return;
        },
        _ => println!("Opção Inválida")
    }


    //println!("Digite enter para continuar...");
    //ler_dados();
    esperar(2);

    }
}