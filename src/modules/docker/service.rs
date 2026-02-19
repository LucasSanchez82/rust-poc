// use futures_util::StreamExt;
use crate::modules::{
    docker::{dto::ContainerDTO, payload::DockerMariadbPayload},
    errors::ServiceError,
    types::ServiceResult,
};

use bollard::{
    Docker,
    container::{Config, CreateContainerOptions},
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

    // pub async fn create(&self, payload: DockerCreatePayload) -> ServiceResult<DockerDTO> {
    //     let config = Config::instance();

    //     let docker_path = PathBuf::from(&config.dockerfile_path).join(payload.dockerfile);
    //     let docker_path_str = docker_path
    //         .to_str()
    //         .ok_or_else(|| ServiceError::bad_request("Bad path"))?;

    //     let image_options = bollard::query_parameters::BuildImageOptionsBuilder::default()
    //         .dockerfile(docker_path_str)
    //         .t("rust-test")
    //         .rm(true)
    //         .build();

    //     // let filename = &args().nth(1).expect("needs first argument");
    //     let archive: File = File::open("archive.example.tar.gz")
    //         .await
    //         .map_err(|_| ServiceError::internal("Failed to open the archive"))?;
    //     let stream = FramedRead::new(archive, BytesCodec::new());
    //     let bytes = stream.try_concat().await.map_err(|error| {
    //         ServiceError::internal("Failed to concat bytes").with_details(error.to_string())
    //     })?;

    //     trace!("Building : {:#?}", image_options);
    //     let mut image_build_stream = self.docker.build_image(
    //         image_options,
    //         None,
    //         Some(http_body_util::Either::Left(Full::new(bytes.freeze()))),
    //     );
    //     while let Ok(Some(msg)) = image_build_stream.try_next().await {
    //         trace!("Message: {msg:#?}");
    //     }

    //     let docker_dto = DockerDTO {
    //         image: "image".to_string(),
    //         name: "name".to_string(),
    //         tag: "tag".to_string(),
    //     };
    //     docker_dto.into()
    // }

    pub async fn run_mariadb(&self, payload: DockerMariadbPayload) -> ServiceResult<ContainerDTO> {
        use bollard::API_DEFAULT_VERSION;
        use bollard::Docker;
        use bollard::container::StartContainerOptions;
        // use bollard::image::CreateImageOptions;
        // use futures_util::stream::TryStreamExt;
        // use std::default::Default;
        let docker = Docker::connect_with_unix(
            "/var/run/docker.sock", // Standard Docker socket path on Linux
            120,                    // Timeout in seconds
            API_DEFAULT_VERSION,
        )
        .expect("Failed to connect to Docker");
        // pull image
        // let options: Option<CreateImageOptions<&str>> = Some(CreateImageOptions {
        //     from_image: "mariadb",
        //     ..Default::default()
        // });
        // let mut stream = docker.create_image(options, None, None);
        // while let Some(msg) = stream.try_next().await.map_err(|error| {
        //     ServiceError::internal("Error during streaming docker").with_details(error.to_string())
        // })? {
        //     if let Some(status) = msg.status {
        //         println!("{}", status);
        //     }
        // }

        //run container
        docker
            .create_container(
                Some(CreateContainerOptions {
                    name: "container_name",
                    ..Default::default()
                }),
                Config {
                    image: Some("mariadb"),
                    ..Default::default()
                },
            )
            .await
            .map_err(|error| {
                ServiceError::internal("Error during streaming docker")
                    .with_details(error.to_string())
            })?;

        docker
            .start_container("container_name", None::<StartContainerOptions<String>>)
            .await
            .map_err(|error| {
                ServiceError::internal("Error during streaming docker")
                    .with_details(error.to_string())
            })?;

        Ok(ContainerDTO {
            id: "container_name".to_string(),
            name: "container_name".to_string(),
        })
    }
}
