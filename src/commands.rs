use bollard::Docker;
use bollard::container::{Config, CreateContainerOptions, ListContainersOptions, LogsOptions, StartContainerOptions};
use bollard::image::CreateImageOptions;
use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::models::HostConfig;
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

pub async fn run_container(image: &str, envs: &[String], volumes: &[String]) -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;

    // Convert envs into format ["KEY=VALUE", ...]
    let environment = if !envs.is_empty() {
        Some(envs.iter().map(|s| &**s).collect::<Vec<&str>>())
    } else {
        None
    };

    // Convert volumes into format ["/host/path:/container/path", ...]
    let host_config = if !volumes.is_empty() {
        Some(HostConfig {
            binds: Some(volumes.iter().map(|s| s.to_string()).collect()),
            ..Default::default()
        })
    } else {
        None
    };

    let create_response = docker.create_container(
        Some(CreateContainerOptions {
            name: "my_rust_container3",
        }),
        Config {
            image: Some(image),
            env: environment,
            host_config,
            tty: Some(true),
            ..Default::default()
        },
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

pub async fn exec_in_container(container_id: &str, cmd: &[String]) -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;

    // Similar to 'docker exec'
    let exec = docker.create_exec(
        container_id,
        CreateExecOptions {
            cmd: Some(cmd.to_vec()),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            tty: Some(false),
            ..Default::default()
        }
    ).await?;

    // Start the exec session and capture output
    let start_result = docker.start_exec(&exec.id, None).await?;

    match start_result {
        StartExecResults::Attached { mut output, .. } => {
            // Stream output
            while let Some(msg) = output.try_next().await? {
                print!("{}", msg);
            }
        }
        StartExecResults::Detached => {
            println!("Command executed in detached mode");
        }
    }

    Ok(())
}