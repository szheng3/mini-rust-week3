use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;
use serde::Deserialize;


use exitfailure::ExitFailure;
use std::thread;



#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}


#[derive(Deserialize)]
struct Info {
    context: String,
}


#[get("/api/health")]
async fn api_health_handler() -> HttpResponse {
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: "Health Check".to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[get("/")]
async fn api_health_handler2() -> HttpResponse {
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: "Health Check".to_string(),
    };
    HttpResponse::Ok().json(response_json)
}


use std::sync::Once;
use actix_web::rt::Runtime;


static INIT_MODEL: Once = Once::new();




#[actix_web::main]
async fn main() -> Result<(), ExitFailure> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    // let mut rt = Runtime::new().unwrap();
    // rt.block_on(async {
    //     init_summarization_model();
    //     drop(rt); // this line causes the error
    // });



    println!("Server started successfully");

    HttpServer::new(move || {
        App::new()
            .service(api_health_handler)
            .service(api_health_handler2)
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await?;
    Ok(())
}

