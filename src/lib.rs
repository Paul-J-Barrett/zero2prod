use actix_web::{web,App,HttpRequest,HttpServer,Responder, HttpResponse};
use std::str::FromStr;


pub async fn run() -> Result<(),std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/healthcheck",web::get().to(health_check))
            .route("/",web::get().to(greet))
            .route("/{name}",web::get().to(greet))
            .route("/greet/{name}",web::get().to(greet))
            .route("/double/{value}",web::get().to(dblvalue))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn greet(req:HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("hello {}",&name)
}

async fn health_check(_req:HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn dblvalue(req:HttpRequest) -> impl Responder {
    let value:i32 = FromStr::from_str( req.match_info().get("value").unwrap_or("1")).unwrap();
    format!("double:{}",&value*2)
}



