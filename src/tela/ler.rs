use std::io;

pub fn ler_dados() -> String{
    let mut dados: String = String::new();
    io::stdin().read_line(&mut dados).expect("Falha ao ler os dados");
    dados.trim().to_string()
}

pub fn ler_dados_int() -> usize {
    loop {
        let mut dados = String::new();

        io::stdin().read_line(&mut dados).expect("Falha ao ler os dados");

        match dados.trim().parse::<usize>() {
            Ok(valor) => return valor,
            Err(_) => {
                println!("Entrada inválida! Digite um número:");
            }
        }
    }
}