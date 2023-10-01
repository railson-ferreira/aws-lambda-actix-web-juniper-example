use lambda_web::actix_web::{self, get, web, App, HttpServer, Responder};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: Option<String>,
}

#[get("/")]
async fn hello(info: web::Query<Info>) -> impl Responder {
    if info.name.is_none() {
        return format!("Hello World!");
    }
    format!("Hello {}!", info.name.as_ref().unwrap())
}

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    let factory = move || App::new().service(hello);
    if is_running_on_lambda() {
        // Run on AWS Lambda
        run_actix_on_lambda(factory).await?;
    } else {
        // Run local server
        println!("Running on 0.0.0.0:8080");
        HttpServer::new(factory)
            .bind("0.0.0.0:8080")?
            .run()
            .await?;
    }
    Ok(())
}