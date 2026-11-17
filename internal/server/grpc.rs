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

        if req.name.is_empty() || req.data.is_empty() || req.content_type.is_empty() {
            return Err(Status::invalid_argument(
                "name, data, and content_type are required",
            ));
        }

        let blob = self
            .repo
            .create(&req)
            .await
            .map_err(|e| Status::internal(format!("failed to create blob: {e}")))?;

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
            .map_err(|e| Status::internal(format!("failed to get blob: {e}")))?
            .ok_or_else(|| Status::not_found("blob not found"))?;

        Ok(Response::new(proto::GetBlobResponse { blob: Some(blob) }))
    }

    async fn update_blob(
        &self,
        request: Request<proto::UpdateBlobRequest>,
    ) -> Result<Response<proto::UpdateBlobResponse>, Status> {
        let req = request.into_inner();

        if req.name.is_empty() || req.data.is_empty() || req.content_type.is_empty() {
            return Err(Status::invalid_argument(
                "name, data, and content_type are required",
            ));
        }

        let id = parse_id(&req.id)?;

        let blob = self
            .repo
            .update(id, &req)
            .await
            .map_err(|e| Status::internal(format!("failed to update blob: {e}")))?
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
            .map_err(|e| Status::internal(format!("failed to delete blob: {e}")))?
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

        if req.limit <= 0 {
            return Err(Status::invalid_argument("limit must be greater than 0"));
        }

        let blobs = self
            .repo
            .list(&req)
            .await
            .map_err(|e| Status::internal(format!("failed to get blobs: {e}")))?;

        Ok(Response::new(proto::ListBlobsResponse { blobs }))
    }
}

fn parse_id(id: &str) -> Result<Uuid, Status> {
    if id.is_empty() {
        return Err(Status::invalid_argument("id is required"));
    }

    Uuid::parse_str(id).map_err(|_| Status::invalid_argument("invalid id"))
}
