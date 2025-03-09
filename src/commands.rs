use bollard::Docker;
use bollard::container::ListContainersOptions;
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