use ceprustico::busca_cep;


fn main() {
    let cep = "01310000";
    let resultado = busca_cep(cep);

    match resultado {
        Ok(response) => println!("Resposta: {:#?}", response),
        Err(e) => eprintln!("Erro ao buscar CEP: {}", e),
    }

}
