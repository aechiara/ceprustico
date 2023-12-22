# ceprustico

Um crate para busca endereço a partir de um CEP no site dos Correios

```rust
use ceprustico::busca_cep;

fn main() {
    let cep = "01310000";
    let resultado = busca_cep(cep);

    match resultado {
        Ok(response) => println!("Resposta: {:#?}", response),
        Err(e) => eprintln!("Erro ao buscar CEP: {}", e),
    }

}

$ cargo run
Resposta: CEP {
    uf: "SP",
    localidade: "São Paulo",
    logradouro: "Avenida Paulista - até 610 - lado par",
    bairro: "Bela Vista",
    cep: "01310000",
}

```