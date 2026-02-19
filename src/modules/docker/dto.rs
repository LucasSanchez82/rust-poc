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
pub struct ContainerDTO {
    pub id: String,
    pub name: String,
}

impl Into<ApiResponse<ContainerDTO>> for ContainerDTO {
    fn into(self) -> ApiResponse<ContainerDTO> {
        Ok(Json(self))
    }
}
