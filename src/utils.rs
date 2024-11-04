extern crate serde_hjson;
use crate::config::DevContainerSpec;
use bollard::container::{RemoveContainerOptions, StopContainerOptions};
use bollard::Docker;

use bollard::exec::{CreateExecOptions, ResizeExecOptions, StartExecResults};
use futures_util::StreamExt;
use std::io::{stdout, Read, Write};
use std::path::PathBuf;
use std::time::Duration;
#[cfg(not(windows))]
use termion::raw::IntoRawMode;
#[cfg(not(windows))]
use termion::{async_stdin, terminal_size};
use tokio::io::AsyncWriteExt;
use tokio::task::spawn;
use tokio::time::sleep;

pub(crate) fn get_dev_container_spec(dev_container_spec_file: PathBuf) -> DevContainerSpec {
    let file = std::fs::File::open(dev_container_spec_file).unwrap();
    serde_hjson::from_reader(file).expect("Invalid JSON")
}

pub(crate) async fn stop(dev_container_spec: &DevContainerSpec, client: &Docker) {
    let _ = client
        .stop_container(
            &dev_container_spec.get_name(),
            Some(StopContainerOptions::default()),
        )
        .await;
}

pub(crate) async fn remove(dev_container_spec: &DevContainerSpec, client: &Docker) {
    let _remove_container = client
        .remove_container(
            &dev_container_spec.get_name(),
            Some(RemoveContainerOptions::default()),
        )
        .await;
}

pub(crate) async fn down(dev_container_spec: &DevContainerSpec, client: &Docker) {
    stop(dev_container_spec, client).await;
    remove(dev_container_spec, client).await;
}

pub(crate) async fn exec(dev_container_spec: &DevContainerSpec, client: &Docker, cmd: Vec<&str>) {
    let tty_size = terminal_size().unwrap();

    let exec = client
        .create_exec(
            &dev_container_spec.get_name(),
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                attach_stdin: Some(true),
                tty: Some(true),
                cmd: Some(cmd),
                ..Default::default()
            },
        )
        .await
        .unwrap()
        .id;
    if let StartExecResults::Attached {
        mut output,
        mut input,
    } = client.start_exec(&exec, None).await.unwrap()
    {
        // pipe stdin into the client exec stream input
        spawn(async move {
            let mut stdin = async_stdin().bytes();
            loop {
                if let Some(Ok(byte)) = stdin.next() {
                    input.write_all(&[byte]).await.ok();
                } else {
                    sleep(Duration::from_nanos(10)).await;
                }
            }
        });

        client
            .resize_exec(
                &exec,
                ResizeExecOptions {
                    height: tty_size.1,
                    width: tty_size.0,
                },
            )
            .await
            .unwrap();

        // set stdout in raw mode so we can do tty stuff
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();

        // pipe client exec output into stdout
        while let Some(Ok(output)) = output.next().await {
            stdout.write_all(output.into_bytes().as_ref()).unwrap();
            stdout.flush().unwrap();
        }
    }
}
