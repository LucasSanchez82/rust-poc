use axum::Json;
use serde::Serialize;

use crate::modules::types::{ApiResponse, ServiceResult};

#[derive(Serialize)]
pub struct DockerDTO {
    pub image: String,
    pub tag: String,
    pub name: String,
}

impl Into<ServiceResult<DockerDTO>> for DockerDTO {
    fn into(self) -> ServiceResult<DockerDTO> {
        let test = String::from("Test");
        Ok(DockerDTO {
            image: test.clone(),
            tag: test.clone(),
            name: test,
        })
    }
}

impl Into<ApiResponse<DockerDTO>> for DockerDTO {
    fn into(self) -> ApiResponse<DockerDTO> {
        Ok(Json(self))
    }
}

#[derive(Serialize)]
pub struct CreateContainerDTO {
    pub id: String,
    pub name: String,
}

impl Into<ApiResponse<CreateContainerDTO>> for CreateContainerDTO {
    fn into(self) -> ApiResponse<CreateContainerDTO> {
        Ok(Json(self))
    }
}
#[derive(Serialize)]
pub struct GetContainerStatusDTO {
    pub status: String,
}

impl Into<ApiResponse<GetContainerStatusDTO>> for GetContainerStatusDTO {
    fn into(self) -> ApiResponse<GetContainerStatusDTO> {
        Ok(Json(self))
    }
}
