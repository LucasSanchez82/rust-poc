use axum::{Router, extract::State, routing::post};

use crate::{
    modules::{
        docker::{
            dto::{ContainerDTO, DockerDTO},
            payload::{DockerCreatePayload, DockerMariadbPayload},
            service::DockerService,
        },
        states::AppState,
        types::ApiResponse,
    },
    utils::extractor::ExtractValidated,
};

pub fn docker_router() -> Router<AppState> {
    Router::new()
        // .route("/", post(handle_create_docker))
        .route("/mariadb", post(handle_run_mariadb))
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
) -> ApiResponse<ContainerDTO> {
    let docker_service = DockerService::new(&app_state.connection);
    docker_service.run_mariadb(payload).await?.into()
}
