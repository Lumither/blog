use std::str::FromStr;

use axum::extract::{Path, State};
use axum::Json;
use chrono::NaiveDateTime;
use serde_json::{json, Map, Value};
use sqlx::{PgPool, query, Row};
use uuid::Uuid;

pub async fn get_post(
    State(db_connection): State<PgPool>,
    Path(post_id): Path<String>,
) -> Result<Json<Value>, Json<Value>> {
    let meta = query(
        r#"
SELECT *
FROM Post
WHERE post_id = $1;
    "#,
    )
        .bind(Uuid::from_str(post_id.as_str()).expect("Failed to parse UUID"))
        .fetch_one(&db_connection)
        .await
        .expect("Failed to execute db query");

    let tags = query(
        r#"
SELECT tag FROM Tag WHERE post_id = $1;
    "#,
    )
        .bind(Uuid::from_str(post_id.as_str()).expect("Failed to parse UUID"))
        .fetch_all(&db_connection)
        .await
        .expect("Failed to execute db query");

    let mut res = Map::new();

    res.insert(
        "title".to_string(),
        json!(meta.get::<String, &str>("title")),
    );
    res.insert(
        "summary".to_string(),
        json!(meta.get::<String, &str>("summary")),
    );
    res.insert(
        "content".to_string(),
        json!(meta.get::<String, &str>("content")),
    );
    res.insert(
        "last_update".to_string(),
        json!(meta.get::<NaiveDateTime, &str>("last_update")),
    );
    res.insert(
        "first_update".to_string(),
        json!(meta.get::<NaiveDateTime, &str>("first_update")),
    );

    let tags: Vec<String> = tags
        .iter()
        .map(|tag| {
            let tag: String = tag.get("tag");
            tag
        })
        .collect();
    res.insert("tags".to_string(), Value::from(tags));

    Ok(Json::from(Value::from(res)))
}
