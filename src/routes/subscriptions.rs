use actix_web::{post, web, HttpResponse};
use chrono::Local;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct FormData {
  email: String,
  name: String,
}

#[post("/subscriptions")]
async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
  let dt = Local::now();
  let naive_utc = dt.naive_utc();
  let request_id = Uuid::new_v4();

  let request_span = tracing::info_span!(
  "Adding a new subscriber.",
  %request_id,
  subscriber_email = %form.email,
  subscriber_name= %form.name
  );

  let _request_span_guard = request_span.enter();

  tracing::info!(
    "request_id {} - Saving new subscriber details in the database",
    request_id
  );
  match sqlx::query!(
    r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
    Uuid::new_v4(),
    form.email,
    form.name,
    naive_utc
  )
  .execute(pool.as_ref())
  .await
  {
    Ok(_) => {
      tracing::info!(
        "request_id {} - New subscriber details have been saved",
        request_id
      );
      HttpResponse::Ok().finish()
    }
    Err(e) => {
      tracing::error!(
        "request_id {} - Failed to execute query: {:?}",
        request_id,
        e
      );
      HttpResponse::InternalServerError().finish()
    }
  }
}
