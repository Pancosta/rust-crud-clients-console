use crate::models::cliente::Cliente;
use super::ler::ler_dados;

pub fn incluir_cliente(){
    let mut cliente: Cliente = Cliente::default();
    //cliente.id = 
    cliente.nome = ler_dados();
    cliente.cpf = ler_dados();
    cliente.endereco = ler_dados();
}