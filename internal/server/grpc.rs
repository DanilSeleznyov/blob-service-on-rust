use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::{
    internal::repo::BlobRepository,
    proto::{self, blob_service_server::BlobService},
};

pub struct BlobServer {
    repo: BlobRepository,
}

impl BlobServer {
    pub fn new(repo: BlobRepository) -> Self {
        Self { repo }
    }
}

#[tonic::async_trait]
impl BlobService for BlobServer {
    async fn create_blob(
        &self,
        request: Request<proto::CreateBlobRequest>,
    ) -> Result<Response<proto::CreateBlobResponse>, Status> {
        let req = request.into_inner();

        if req.name.is_empty() || req.content_type.is_empty() {
            return Err(Status::invalid_argument(
                "name and content_type are required",
            ));
        }

        let blob = self
            .repo
            .create(&req)
            .await
            .map_err(|e| db_status("create blob", e))?;

        Ok(Response::new(proto::CreateBlobResponse {
            blob: Some(blob),
        }))
    }

    async fn get_blob(
        &self,
        request: Request<proto::GetBlobRequest>,
    ) -> Result<Response<proto::GetBlobResponse>, Status> {
        let req = request.into_inner();
        let id = parse_id(&req.id)?;

        let blob = self
            .repo
            .get(id)
            .await
            .map_err(|e| db_status("get blob", e))?
            .ok_or_else(|| Status::not_found("blob not found"))?;

        Ok(Response::new(proto::GetBlobResponse { blob: Some(blob) }))
    }

    async fn update_blob(
        &self,
        request: Request<proto::UpdateBlobRequest>,
    ) -> Result<Response<proto::UpdateBlobResponse>, Status> {
        let req = request.into_inner();
        let id = parse_id(&req.id)?;

        if matches!(req.name.as_deref(), Some(""))
            || matches!(req.content_type.as_deref(), Some(""))
        {
            return Err(Status::invalid_argument(
                "name and content_type must not be empty",
            ));
        }

        if req.name.is_none()
            && req.content_type.is_none()
            && req.data.is_none()
            && req.tags.is_empty()
            && req.metadata.is_empty()
        {
            return Err(Status::invalid_argument("nothing to update"));
        }

        let blob = self
            .repo
            .update(id, &req)
            .await
            .map_err(|e| db_status("update blob", e))?
            .ok_or_else(|| Status::not_found("blob not found"))?;

        Ok(Response::new(proto::UpdateBlobResponse {
            blob: Some(blob),
        }))
    }

    async fn delete_blob(
        &self,
        request: Request<proto::DeleteBlobRequest>,
    ) -> Result<Response<proto::DeleteBlobResponse>, Status> {
        let req = request.into_inner();
        let id = parse_id(&req.id)?;

        let blob = self
            .repo
            .delete(id)
            .await
            .map_err(|e| db_status("delete blob", e))?
            .ok_or_else(|| Status::not_found("blob not found"))?;

        Ok(Response::new(proto::DeleteBlobResponse {
            blob: Some(blob),
        }))
    }

    async fn list_blobs(
        &self,
        request: Request<proto::ListBlobsRequest>,
    ) -> Result<Response<proto::ListBlobsResponse>, Status> {
        let req = request.into_inner();

        if !(1..=100).contains(&req.limit) {
            return Err(Status::invalid_argument("limit must be between 1 and 100"));
        }

        let blobs = self
            .repo
            .list(&req)
            .await
            .map_err(|e| db_status("get blobs", e))?;

        Ok(Response::new(proto::ListBlobsResponse { blobs }))
    }
}

fn parse_id(id: &str) -> Result<Uuid, Status> {
    if id.is_empty() {
        return Err(Status::invalid_argument("id is required"));
    }

    Uuid::parse_str(id).map_err(|_| Status::invalid_argument("invalid id"))
}

fn db_status(action: &str, error: sqlx::Error) -> Status {
    if let sqlx::Error::Database(db) = &error {
        if db.code().as_deref() == Some("22001") {
            return Status::invalid_argument("field value is too long");
        }
    }

    eprintln!("failed to {action}: {error}");
    Status::internal(format!("failed to {action}"))
}
