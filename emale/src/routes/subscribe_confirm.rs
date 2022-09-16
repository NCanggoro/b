use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(
	skip(param, pool)
	name = "Confirm a pending subscriber"
)]
pub async fn confirm(
	param: web::Query<Parameters>, 
	pool: web::Data<PgPool>
) -> HttpResponse {
    let id = match get_subcriber_id_from_token(
			&pool, 
			&param.subscription_token
		)
		.await 
		{
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match id {
        None => HttpResponse::Unauthorized().finish(),
        Some(subs_id) => {
            if confirm_subscriber(subs_id, &pool).await.is_err() {
                return HttpResponse::InternalServerError().finish();
            }
			HttpResponse::Ok().finish()
		}
	}
}

#[tracing::instrument(
	name = "Change status to confirmed", 
	skip(pool, subs_id)
)]
pub async fn confirm_subscriber(
	subs_id: Uuid, 
	pool: &PgPool
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriber SET status = 'confirmed' WHERE id = $1"#,
        subs_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}

#[tracing::instrument(
	name = "Get subs id from token",
	skip(pool, subs_token)
)]
pub async fn get_subcriber_id_from_token(
	pool: &PgPool,
	subs_token: &str
) -> Result<Option<Uuid>, sqlx::Error> {
	let result = sqlx::query!(
		r#"SELECT subscriber_id FROM subscriber_tokens WHERE subscriber_token = $1"#,
		subs_token
	)
	.fetch_optional(pool)
	.await
	.map_err(|e| {
		tracing::error!("Failed to execute query: {:?}", e);
		e
	})?;
	Ok(result.map(|r| r.subscriber_id))
}