use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().json("OK")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  println!("Running server on port :3002");
  HttpServer::new(|| {
      App::new()
          .route("/", web::get().to(index))
  })
  .bind("127.0.0.1:3002")?
  .run()
  .await
}
