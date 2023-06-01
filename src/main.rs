// RustyWebApp
mod api;
mod model;
mod repository;

use api::task::{get_task, start_task, submit_task, fail_task, complete_task, pause_task};
use repository::ddb::DDBRepository;
use actix_web::{HttpServer, App, web::Data, middleware::Logger};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // Actix-web logging variables
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // AWS config
    let config = aws_config::load_from_env().await;
    
    // HTTP server struct
    HttpServer::new(move || {
        let ddb_repo:DDBRepository = DDBRepository::init(String::from("task"), config.clone(), ); 
        let ddb_data = Data::new(ddb_repo);
    
        App::new()
            .wrap(Logger::default())
            .app_data(ddb_data)
            .service(get_task)
            .service(start_task)
            .service(submit_task)
            .service(fail_task)
            .service(complete_task)
            .service(pause_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
