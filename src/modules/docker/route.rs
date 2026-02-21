use axum::{
    Router,
    extract::State,
    routing::{get, post},
};

use crate::{
    modules::{
        docker::{
            dto::{CreateContainerDTO, GetContainerStatusDTO},
            payload::{DockerMariadbPayload, GetContainerStatusPayload},
            service::DockerService,
        },
        states::AppState,
        types::ApiResponse,
    },
    utils::extractor::{ExtractValidated, ExtractValidatedParams},
};

pub fn docker_router() -> Router<AppState> {
    Router::new()
        // .route("/", post(handle_create_docker))
        .route("/mariadb", post(handle_run_mariadb))
        .route("/mariadb", get(handle_get_container_status_mariadb))
}

// async fn handle_create_docker(
//     app_state: State<AppState>,
//     ExtractValidated(payload): ExtractValidated<DockerCreatePayload>,
// ) -> ApiResponse<DockerDTO> {
//     let docker_service = DockerService::new(&app_state.connection);
//     docker_service.create(payload).await?.into()
// }

async fn handle_run_mariadb(
    app_state: State<AppState>,
    ExtractValidated(payload): ExtractValidated<DockerMariadbPayload>,
) -> ApiResponse<CreateContainerDTO> {
    let docker_service = DockerService::new(&app_state.connection);
    docker_service
        .create_and_start_container(payload)
        .await?
        .into()
}

#[axum::debug_handler]
async fn handle_get_container_status_mariadb(
    app_state: State<AppState>,
    ExtractValidatedParams(payload): ExtractValidatedParams<GetContainerStatusPayload>,
) -> ApiResponse<GetContainerStatusDTO> {
    let docker_service = DockerService::new(&app_state.connection);

    docker_service.get_container_status(payload).await?.into()
}
