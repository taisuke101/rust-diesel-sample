#[macro_use]
extern crate diesel;

mod actions;
mod models;
mod schema;

use actix_web::{get, post, App, Error, HttpRequest, HttpResponse, HttpServer, web, Responder, middleware};
use diesel::{PgConnection, r2d2::{self, ConnectionManager}};
use uuid::Uuid;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn hello(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", &name)
}

#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<DbPool>,
    user_uuid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uuid = user_uuid.into_inner();
    let connection = pool.get().expect("connection failed");

    let user = web::block(move || actions::find_user_by_uuid(user_uuid, &connection))
    .await
    .map_err(|err| {
        eprintln!("{}", err);
        HttpResponse::InternalServerError().finish()
    })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound()
        .body(format!("No user found with uuid: {}", user_uuid));
        Ok(res)
    }
}

#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("connection failed");

    let user = web::block(move || actions::insert_new_user(&form.name, &connection))
    .await
    .map_err(|err| {
        eprintln!("{}", err);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("connection failed");
    let port = std::env::var("PORT").expect("PORT");

    let conn_manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
    .build(conn_manager)
    .expect("failed to create pool");

    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
        .route("/", web::get().to(hello))
        .route("/{name}", web::get().to(hello))
        .service(get_user)
        .service(add_user)
        .wrap(middleware::Logger::default())
    }).bind(port)?
    .run()
    .await
}

