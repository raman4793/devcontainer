use crate::attach::attach;
use crate::config::DevContainerSpec;
use bollard::{
    container::{Config, CreateContainerOptions},
    Docker,
};
#[allow(dead_code, unused, unused_imports)]
use std::{collections::HashMap, path::PathBuf};

#[cfg(not(windows))]
pub(crate) async fn run(dev_container_spec: &DevContainerSpec, client: &Docker) {
    let container_config = CreateContainerOptions {
        name: dev_container_spec.get_name(),
        platform: None,
    };
    let command = dev_container_spec.command.clone();
    let command: Vec<String> = command.split_whitespace().map(str::to_string).collect();
    let id = client
        .create_container(
            Some(container_config),
            Config {
                image: Some(dev_container_spec.get_image_name()),
                cmd: Some(command),
                ..Default::default()
            },
        )
        .await
        .unwrap()
        .id;
    client.start_container::<String>(&id, None).await.unwrap();
    attach(dev_container_spec, client).await;
}
