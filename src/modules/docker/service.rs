use crate::modules::{
    docker::{
        dto::{CreateContainerDTO, GetContainerStatusDTO},
        payload::{DockerMariadbPayload, GetContainerStatusPayload},
    },
    errors::ServiceError,
    types::ServiceResult,
};

use bollard::container::StartContainerOptions;
use bollard::{
    Docker,
    container::{Config, CreateContainerOptions, InspectContainerOptions},
};
use sea_orm::DatabaseConnection;

pub struct DockerService<'a> {
    pub db: &'a DatabaseConnection,
    pub docker: Docker,
}

impl<'a> DockerService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        let docker = Docker::connect_with_socket_defaults().unwrap();
        Self { db, docker }
    }

    pub async fn create_and_start_container(
        &self,
        payload: DockerMariadbPayload,
    ) -> ServiceResult<CreateContainerDTO> {
        let env_root_password = format!("MARIADB_ROOT_PASSWORD={}", payload.root_password);
        // let env_password = format!("MARIADB_PASSWORD={}", payload.password);
        // let env_user = format!("MARIADB_USER={}", payload.user);
        // let env_database_name = format!("MARIADB_DATABASE={}", payload.database_name);

        let container_res = self
            .docker
            .create_container(
                Some(CreateContainerOptions {
                    name: "container_name",
                    ..Default::default()
                }),
                Config {
                    image: Some("mariadb"),
                    env: Some(vec![&env_root_password]),
                    ..Default::default()
                },
            )
            .await?;

        let docker = self.docker.clone();
        tokio::task::spawn(async move {
            docker
                .start_container("container_name", None::<StartContainerOptions<String>>)
                .await?;

            Ok::<(), ServiceError>(())
        });

        Ok(CreateContainerDTO {
            id: container_res.id,
            name: "container_name".to_string(),
        })
    }

    pub async fn get_container_status(
        &self,
        payload: GetContainerStatusPayload,
    ) -> ServiceResult<GetContainerStatusDTO> {
        // TODO: Check if the container exist
        let options = InspectContainerOptions::default();

        let res = self
            .docker
            .inspect_container(&payload.container_name, Some(options))
            .await?;

        let unknown_value = "UNKNOWN".to_owned();
        Ok(GetContainerStatusDTO {
            status: res
                .state
                .map(|value| {
                    value
                        .status
                        .map(|v| format!("{:#?}", v))
                        .unwrap_or(unknown_value.clone())
                })
                .unwrap_or(unknown_value),
        })
    }
}
