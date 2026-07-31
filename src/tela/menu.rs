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
            2 - Listar clientes\n\
            3 - Editar cliente\n\
            4 - Excluir cliente\n\
            0 - Sair do programa\n\
        ");

    let opcao = ler_dados_int();

    match opcao{
        1 => incluir_cliente(clientes),
        2 => listar_clientes(clientes),
        3 => alterar_cliente(clientes),
        4 => excluir_cliente(clientes),
        0 => {
            println!("Finalizando ...");
            return;
        },
        _ => println!("Opção Inválida!")
    }

    }
}