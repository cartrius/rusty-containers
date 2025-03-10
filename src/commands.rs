use bollard::Docker;
use bollard::container::{Config, CreateContainerOptions, ListContainersOptions, LogsOptions, StartContainerOptions};
use bollard::image::CreateImageOptions;
use futures_util::stream::TryStreamExt;
use std::default::Default;
use anyhow::Result;

pub async fn list_containers() -> Result<()> {
    // Client will connect to the standard unix socket location
    let docker = Docker::connect_with_local_defaults()?;
    let containers = docker.list_containers(Some(ListContainersOptions::<String> {
        all: true, // Show all containers
        ..Default::default()
    })).await?;

    for container in containers {
        println!(
            "ID: {:?}, Image: {:?}, Status: {:?}",
            container.id,
            container.image,
            container.status
        );
    }

    Ok(())
}

pub async fn pull_image(image: &str) -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;

    let mut pull_stream = docker.create_image(
        Some(CreateImageOptions {
            from_image: image,
            ..Default::default()
        }),
        None, // Auth config
        None // Registry config
    );

    while let Some(progress) = pull_stream.try_next().await? {
        if let Some(status) = progress.status {
            println!("Pull status: {}", status);
        }
    }

    println!("Finished pulling image: {}", image);
    Ok(())
}

pub async fn run_container(image: &str) -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;

    let create_response = docker.create_container(
        Some(CreateContainerOptions {
            name: "my_rust_container2",
        }),
        Config {
            image: Some(image),
            tty: Some(true),
            // Enivronment variables, volumes, etc here
            ..Default::default()
        }
    ).await?;

    let id = create_response.id;
    println!("Created container with ID: {}", id);

    // Start container
    docker
        .start_container(&id, None::<StartContainerOptions<String>>)
        .await?;
    println!("Container started successfully!");

    Ok(())
}

pub async fn stop_container(container_id: &str) -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;
    docker.stop_container(container_id, None).await?;
    println!("Stopped container: {}", container_id);
    Ok(())
}

pub async fn logs_container(container_id: &str, follow: bool) -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;

    let mut logs_stream = docker.logs(
        container_id,
        Some(LogsOptions::<String> {
            follow,
            stdout: true,
            stderr: true,
            tail: "all".to_string(),
            ..Default::default()
        })
    );

    // Each frame is a chunk of log data
    while let Some(log_result) = logs_stream.try_next().await? {
        // log_result may be plain text or contain metadata
        println!("log {}", log_result);
    }

    Ok(())
}