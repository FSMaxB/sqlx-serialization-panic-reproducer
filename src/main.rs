use serde::{Deserialize, Serialize};
use sqlx::{Connection, PgConnection};
use sqlx::types::Json;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let mut connection = sqlx::PgConnection::connect("postgresql://postgres:postgres@localhost:14538").await?;

    // this goes through successfully
    insert_variant(&mut connection, Variant::A).await?;

    // this panics
    insert_variant(&mut connection, Variant::Unknown).await?;
    // And yes, writing code like this should be considered a bug, because this
    // can obviously never work with a non-serializable variant, but I would
    // strongly argue that this should not panic (especially since this is an easy
    // mistake to make)

    Ok(())
}

async fn insert_variant(connection: &mut PgConnection, variant: Variant) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT INTO variants (variant) VALUES ($1)",
        Json(variant) as _,
    ).execute(connection).await?;

    Ok(())
}

#[derive(Clone, Copy, Deserialize, Serialize)]
enum Variant {
    A,
    B,
    C,
    #[serde(other, skip_serializing)]
    Unknown,
}