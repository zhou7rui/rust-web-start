use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Local;

use crate::{
    modle::User,
    schema::{CreateUserSchema, FilterOptions},
    AppState,
};

#[get("/users")]
async fn user_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);

    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let result = sqlx::query_as::<_, User>("SELECT * FROM users LIMIT $1 OFFSET $2;")
        .bind(limit as i32)
        .bind(offset as i32)
        .fetch_all(&data.db)
        .await
        .unwrap();

    HttpResponse::Ok().json(result)
}

#[get("/users/{id}")]
async fn get_user_handler(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();

    let query_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&data.db)
        .await
        .unwrap();

    HttpResponse::Ok().json(query_result)
}

#[post("/users")]
async fn create_user_handler(
    body: web::Json<CreateUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let user_id = uuid::Uuid::new_v4();

    let now = Local::now();

    let query_result =
        sqlx::query(r#"INSERT INTO users(id,name,created_at,updated_at) VALUES($1,$2,$3,$4)"#)
            .bind(user_id.clone())
            .bind(&body.name)
            .bind(now)
            .bind(now)
            .execute(&data.db)
            .await
            .map_err(|err| err.to_string());

    if let Err(err) = query_result {
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
    }

    let query_result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&data.db)
        .await
        .unwrap();

    HttpResponse::Ok().json(query_result)
}

pub fn api_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(user_list_handler)
        .service(get_user_handler)
        .service(create_user_handler);

    conf.service(scope);
}
