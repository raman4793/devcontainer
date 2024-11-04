use crate::config::{BuildSpec, DevContainerSpec};
use bollard::{image::BuildImageOptions, Docker};
use flate2::{write::GzEncoder, Compression};
use futures_util::StreamExt;
#[allow(dead_code, unused, unused_imports)]
use std::{collections::HashMap, path::PathBuf};
use std::{io::Write, str::FromStr};
use tar::{Builder, Header};

#[cfg(not(windows))]
#[cfg(not(windows))]
use tokio::{fs::File, io::AsyncReadExt};
pub(crate) async fn build(dev_container_spec: &DevContainerSpec, client: &Docker) {
    match &dev_container_spec.build {
        Some(build_spec) => {
            let build_config = BuildImageOptions {
                dockerfile: build_spec.dockerfile.clone(),
                t: dev_container_spec.get_name(),
                ..Default::default()
            };
            let bytes = create_temp_tar_in_memory(build_spec).await;
            let mut build_log = client.build_image(build_config, None, Some(bytes.into()));
            while let Some(msg) = build_log.next().await {
                println!("Message: {msg:?}");
            }
        }
        None => {
            println!("No build config found in the devcontainer.json")
        }
    };
}
async fn create_temp_tar_in_memory(build_spec: &BuildSpec) -> Vec<u8> {
    let dockerfile = PathBuf::from_str(&build_spec.dockerfile).unwrap();
    let tar = create_tar(&dockerfile).await;
    get_bytes_from_tar(tar).await
}

async fn get_bytes_from_tar(tar: Builder<Vec<u8>>) -> Vec<u8> {
    let uncompressed = tar.into_inner().unwrap();
    let mut c = GzEncoder::new(Vec::new(), Compression::default());

    c.write_all(&uncompressed).unwrap();

    c.finish().unwrap()
}

async fn create_tar(dockerfile: &PathBuf) -> Builder<Vec<u8>> {
    let mut header = Header::new_gnu();
    let dockerfile = read_file(dockerfile).await;
    header.set_path("Dockerfile").unwrap();
    header.set_size(dockerfile.len() as u64);
    header.set_mode(0o755);
    header.set_cksum();

    let mut tar = Builder::new(Vec::new());
    tar.append(&header, &dockerfile[..]).unwrap();

    tar
}

async fn read_file(dockerfile: &PathBuf) -> Vec<u8> {
    let mut file = File::open(dockerfile).await.unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).await.unwrap();
    buf
}
