use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use sudachi::analysis::Mode;
use sudachi::analysis::Tokenize;
use sudachi::dic::dictionary::JapaneseDictionary;
use sudachi::config::Config;
use sudachi::error::SudachiError;
use sudachi::analysis::stateless_tokenizer::StatelessTokenizer;
use std::sync::Arc;

#[derive(Deserialize)]
struct TokenizerRequestBody{
  text: String
}

fn tokenize(input: &str, dict: Arc<JapaneseDictionary>) -> Result<serde_json::Value, SudachiError>{
  let tokenizer = StatelessTokenizer::new(&dict);
  let tokens = tokenizer.tokenize(input, Mode::C, false)?;

  Ok(json!(tokens.iter().map(|token| {
    json!({
      "surface": *token.surface(),
      "pos": *token.part_of_speech(),
      "normalized_form": *token.normalized_form(),
      "reading_form": *token.reading_form(),
      "dictionary_form": *token.dictionary_form()
    })
  }).collect::<Vec<_>>()))
}

async fn handle_request(data: web::Json<TokenizerRequestBody>, dict: web::Data<Arc<JapaneseDictionary>>) -> impl Responder{
  match tokenize(&data.text, dict.get_ref().clone()){
    Ok(res) => HttpResponse::Ok().json(res),
    Err(_) => HttpResponse::InternalServerError().body("tokenize err")
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
  let cfg = Config::new(None, None, None).unwrap();
  let dict = Arc::new(JapaneseDictionary::from_cfg(&cfg).unwrap());

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(dict.clone()))
      .route("/tokenize", web::post().to(handle_request))
  })
  .bind("127.0.0.1:2971")?
  .run()
  .await
}
