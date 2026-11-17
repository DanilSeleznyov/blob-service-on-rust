use std::{collections::HashMap, ffi::OsStr, path::Path};

use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use sha2::{Digest, Sha256};
use sqlx::{PgPool, types::Json};
use uuid::Uuid;

use crate::proto;

macro_rules! columns {
    () => {
        "id, name, content_type, extension, data, size, sha256_hash, created_at, \
         updated_at, tags, metadata, owner_id, access_level"
    };
}

#[derive(sqlx::FromRow)]
struct BlobRow {
    id: Uuid,
    name: String,
    content_type: String,
    extension: String,
    data: Vec<u8>,
    size: i64,
    sha256_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    tags: Option<Vec<String>>,
    metadata: Option<Json<HashMap<String, String>>>,
    owner_id: Option<String>,
    access_level: Option<String>,
}

impl From<BlobRow> for proto::Blob {
    fn from(row: BlobRow) -> Self {
        Self {
            id: row.id.to_string(),
            name: row.name,
            content_type: row.content_type,
            extension: row.extension,
            data: row.data,
            size: row.size,
            sha256_hash: row.sha256_hash,
            created_at: Some(to_timestamp(row.created_at)),
            updated_at: Some(to_timestamp(row.updated_at)),
            tags: row.tags.unwrap_or_default(),
            metadata: row.metadata.map(|json| json.0).unwrap_or_default(),
            owner_id: row.owner_id.unwrap_or_default(),
            access_level: row.access_level.unwrap_or_default(),
        }
    }
}

pub struct BlobRepository {
    pool: PgPool,
}

impl BlobRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, req: &proto::CreateBlobRequest) -> Result<proto::Blob, sqlx::Error> {
        let row = sqlx::query_as::<_, BlobRow>(concat!(
            "INSERT INTO blobs (name, content_type, extension, data, size, sha256_hash, tags) ",
            "VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING ",
            columns!()
        ))
        .bind(&req.name)
        .bind(&req.content_type)
        .bind(extension(&req.name))
        .bind(&req.data)
        .bind(req.data.len() as i64)
        .bind(sha256(&req.data))
        .bind(&req.tags)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<proto::Blob>, sqlx::Error> {
        let row = sqlx::query_as::<_, BlobRow>(concat!(
            "SELECT ",
            columns!(),
            " FROM blobs WHERE id = $1"
        ))
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Into::into))
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: &proto::UpdateBlobRequest,
    ) -> Result<Option<proto::Blob>, sqlx::Error> {
        let metadata = (!req.metadata.is_empty()).then(|| Json(req.metadata.clone()));

        let row = sqlx::query_as::<_, BlobRow>(concat!(
            "UPDATE blobs SET name = $1, content_type = $2, extension = $3, data = $4, ",
            "size = $5, sha256_hash = $6, updated_at = NOW(), tags = $7, metadata = $8, ",
            "access_level = $9 WHERE id = $10 RETURNING ",
            columns!()
        ))
        .bind(&req.name)
        .bind(&req.content_type)
        .bind(extension(&req.name))
        .bind(&req.data)
        .bind(req.data.len() as i64)
        .bind(sha256(&req.data))
        .bind(&req.tags)
        .bind(metadata)
        .bind(&req.access_level)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Into::into))
    }

    pub async fn delete(&self, id: Uuid) -> Result<Option<proto::Blob>, sqlx::Error> {
        let row = sqlx::query_as::<_, BlobRow>(concat!(
            "DELETE FROM blobs WHERE id = $1 RETURNING ",
            columns!()
        ))
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Into::into))
    }

    pub async fn list(
        &self,
        req: &proto::ListBlobsRequest,
    ) -> Result<Vec<proto::Blob>, sqlx::Error> {
        let rows = sqlx::query_as::<_, BlobRow>(concat!(
            "SELECT ",
            columns!(),
            " FROM blobs",
            " WHERE ($1 = '' OR name ILIKE '%' || $1 || '%')",
            " AND ($2 = '' OR content_type = $2)",
            " AND (cardinality($3::varchar[]) = 0 OR tags && $3::varchar[])",
            " LIMIT $4"
        ))
        .bind(&req.name_filter)
        .bind(&req.content_type_filter)
        .bind(&req.tag_filters)
        .bind(i64::from(req.limit))
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }
}

fn extension(name: &str) -> String {
    Path::new(name)
        .extension()
        .and_then(OsStr::to_str)
        .map(|ext| format!(".{ext}"))
        .unwrap_or_default()
}

fn sha256(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}

fn to_timestamp(dt: DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}
