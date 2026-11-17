use std::{collections::HashMap, ffi::OsStr, path::Path};

use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use sha2::{Digest, Sha256};
use sqlx::{PgPool, types::Json};
use uuid::Uuid;

use crate::proto;

macro_rules! columns {
    () => {
        "id, name, content_type, extension, size, sha256_hash, created_at, \
         updated_at, tags, metadata, owner_id, access_level"
    };
}

#[derive(sqlx::FromRow)]
struct BlobRow {
    id: Uuid,
    name: String,
    content_type: String,
    extension: String,
    #[sqlx(default)]
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
        let metadata = (!req.metadata.is_empty()).then(|| Json(req.metadata.clone()));

        let row = sqlx::query_as::<_, BlobRow>(concat!(
            "INSERT INTO blobs (name, content_type, extension, data, size, sha256_hash, tags, ",
            "metadata, owner_id, access_level) ",
            "VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING ",
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
        .bind((!req.owner_id.is_empty()).then_some(req.owner_id.as_str()))
        .bind((!req.access_level.is_empty()).then_some(req.access_level.as_str()))
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<proto::Blob>, sqlx::Error> {
        let row = sqlx::query_as::<_, BlobRow>(concat!(
            "SELECT ",
            columns!(),
            ", data FROM blobs WHERE id = $1"
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
        let new_extension = req.name.as_deref().map(extension);
        let new_size = req.data.as_ref().map(|data| data.len() as i64);
        let new_hash = req.data.as_deref().map(sha256);
        let metadata = (!req.metadata.is_empty()).then(|| Json(req.metadata.clone()));

        let row = sqlx::query_as::<_, BlobRow>(concat!(
            "UPDATE blobs SET ",
            "name = COALESCE($1, name), ",
            "content_type = COALESCE($2, content_type), ",
            "extension = COALESCE($3, extension), ",
            "data = COALESCE($4, data), ",
            "size = COALESCE($5, size), ",
            "sha256_hash = COALESCE($6, sha256_hash), ",
            "updated_at = NOW(), ",
            "tags = CASE WHEN cardinality($7::varchar[]) = 0 THEN tags ELSE $7::varchar[] END, ",
            "metadata = COALESCE($8, metadata) ",
            "WHERE id = $9 RETURNING ",
            columns!()
        ))
        .bind(req.name.as_deref())
        .bind(req.content_type.as_deref())
        .bind(new_extension)
        .bind(req.data.as_deref())
        .bind(new_size)
        .bind(new_hash)
        .bind(&req.tags)
        .bind(metadata)
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
            " WHERE ($1 = '' OR position(lower($1) in lower(name)) > 0)",
            " AND ($2 = '' OR content_type = $2)",
            " AND (cardinality($3::varchar[]) = 0 OR tags && $3::varchar[])",
            " ORDER BY created_at DESC, id DESC",
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
