use reqwest::blocking;
use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug)]
pub enum CepError {
    InvalidInput(String),
    HttpRequest(String),
    // outros tipos de erros podem ser adicionados aqui se necessário
}

impl fmt::Display for CepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CepError::InvalidInput(msg) => write!(f, "Erro de entrada: {}", msg),
            CepError::HttpRequest(err) => write!(f, "Erro de requisição HTTP: {}", err),
        }
    }
}

impl From<reqwest::Error> for CepError {
    fn from(err: reqwest::Error) -> CepError {
        CepError::HttpRequest(err.to_string())
    }
}


impl std::error::Error for CepError {}

#[derive(Serialize, Deserialize, Debug)]
struct RespostaAPI {
    erro: bool,
    mensagem: String,
    total: u32,
    dados: Vec<CEP>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CEP {
    uf: String,
    localidade: String,
    #[serde(alias = "logradouroDNEC")]
    logradouro: String,
    bairro: String,
    cep: String,
}

// implements clone values from RespostaCorresios.dados.first
impl Clone for CEP {
    fn clone(&self) -> Self {
        CEP {
            uf: self.uf.clone(),
            localidade: self.localidade.clone(),
            logradouro: self.logradouro.clone(),
            bairro: self.bairro.clone(),
            cep: self.cep.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RespostaCorreios {
    erro: Option<String>,
    dados: Vec<CEP>,
}

// pub async fn busca_cep(cep: &str) -> Result<String, CepError> {
pub fn busca_cep(cep: &str) -> Result<CEP, CepError> {

    // check if cep is null or empty
    if cep.is_empty() {
        return Err(CepError::InvalidInput("CEP inválido. O CEP não pode ser vazio".into()));
    }

    // check if cep is only digits
    if !cep.chars().all(|c| c.is_digit(10)) {
        return Err(CepError::InvalidInput("CEP inválido. O CEP deve conter apenas dígitos".into()));
    }

    // check if cep has size 8
    if cep.len() != 8 {
        return Err(CepError::InvalidInput("CEP inválido. O CEP deve conter 8 dígitos".into()));
    }

    let payload = serde_json::json!({
        "endereco": cep,
        "tipoCEP": "ALL",
        "cepaux": "",
        "mensagem_alerta": "",
        "pagina": "/app/endereco/index.php",
        "cep": cep,
    });

    let client = blocking::Client::new();
    let response = client.post("https://buscacepinter.correios.com.br/app/consulta/html/consulta-detalhes-cep.php")
        .form(&payload)
        .send()?;

    if !response.status().is_success() {
        let error_message = format!("Erro ao buscar CEP: {}", response.status());
        return Err(CepError::HttpRequest(error_message));
    }

    let resposta = serde_json::from_str::<RespostaAPI>(response.text().unwrap().as_str());
    let resposta = resposta.unwrap();

    let cep = resposta.dados.first().unwrap().clone();

    Ok(cep)
}


#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
