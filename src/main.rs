mod models;


use models::cliente::Cliente;

fn main() {
    let cliente: Cliente = Cliente{
        id: 1,
        nome:"teste".to_string(),
        cpf: "212121".to_string(),
        endereco: "212121".to_string(),
    };


    println!("{}", cliente.nome);
}
