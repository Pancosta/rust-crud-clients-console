# App Clientes Console

Aplicação de linha de comando (CLI) desenvolvida em **Rust** para realizar o gerenciamento de clientes em memória.

O sistema permite cadastrar, listar, alterar e excluir clientes através de um menu interativo executado no terminal. O projeto foi desenvolvido com foco no aprendizado da linguagem Rust, utilizando conceitos como módulos, structs, vetores, funções e tratamento de entrada do usuário.

## Funcionalidades

- Cadastro de clientes
- Listagem de clientes cadastrados
- Alteração de clientes por ID
- Exclusão de clientes por ID com confirmação
- Validação de entrada para evitar encerramentos inesperados
- Interface simples em modo texto

## Estrutura do Projeto

```
src
├── main.rs
├── models
│   ├── cliente.rs
│   └── mod.rs
└── tela
    ├── ler.rs
    ├── menu.rs
    ├── operacoes_basicas.rs
    ├── servico_cliente.rs
    └── mod.rs
```

## Tecnologias utilizadas

- Rust
- Cargo
- Crate `clearscreen`

## Pré-requisitos

Antes de executar o projeto é necessário possuir o Rust instalado.

### Linux / macOS

Execute:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Após a instalação:

```bash
source "$HOME/.cargo/env"
```

Verifique se tudo ocorreu corretamente:

```bash
rustc --version
cargo --version
```

### Windows

Baixe o instalador em:

https://www.rust-lang.org/tools/install

Após instalar, abra um novo terminal e execute:

```bash
rustc --version
cargo --version
```

## Clonando o projeto

```bash
git clone https://github.com/Pancosta/rust-crud-clients-console.git
```

Entre na pasta:

```bash
cd app_clientes_console
```

## Instalando as dependências

O Cargo fará isso automaticamente na primeira execução.

Caso queira baixar antes:

```bash
cargo fetch
```

## Executando

Modo normal:

```bash
cargo run
```

Compilando em modo Release:

```bash
cargo run --release
```

Ou somente compilando:

```bash
cargo build
```

## Menu

Ao iniciar a aplicação será exibido um menu semelhante a este:

```
=========== Menu ===========

1 - Cadastrar cliente
2 - Alterar cliente
3 - Excluir cliente
4 - Listar clientes
0 - Sair
```

## Dados armazenados

Cada cliente possui:

- ID
- Nome
- CPF
- Endereço

Atualmente todas as informações permanecem apenas em memória.

Ao fechar o programa todos os dados cadastrados são perdidos.

## Dependências

```toml
[dependencies]
clearscreen = "2.0.1"
```

## Objetivo

Este projeto foi desenvolvido para praticar conceitos fundamentais da linguagem Rust, como:

- Organização em módulos
- Structs
- Vetores (`Vec`)
- Ownership
- Borrowing
- Referências mutáveis
- Tratamento de entrada do usuário
- Estruturas de decisão
- Laços de repetição

## Melhorias futuras

- Persistência em arquivos
- Busca por nome
- Validação de CPF
- Edição individual de campos
- Interface gráfica
- Banco de dados
- Testes automatizados

## Licença

Este projeto é distribuído apenas para fins de estudo.