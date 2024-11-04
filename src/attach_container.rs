use bollard::container::{AttachContainerOptions, AttachContainerResults};
use bollard::Docker;

use futures_util::StreamExt;
use std::io::{stdout, Read, Write};
use std::time::Duration;
#[cfg(not(windows))]
use termion::async_stdin;
#[cfg(not(windows))]
use termion::raw::IntoRawMode;
use tokio::io::AsyncWriteExt;
use tokio::task::spawn;
use tokio::time::sleep;

const IMAGE: &str = "alpine:3";

pub(crate) async fn attach(
    container_name: &str,
    client: Docker,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    #[cfg(not(windows))]
    {
        let AttachContainerResults {
            mut output,
            mut input,
        } = client
            .attach_container(
                container_name,
                Some(AttachContainerOptions::<String> {
                    stdout: Some(true),
                    stderr: Some(true),
                    stdin: Some(true),
                    stream: Some(true),
                    ..Default::default()
                }),
            )
            .await?;

        // pipe stdin into the docker attach stream input
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

        // set stdout in raw mode so we can do tty stuff
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode()?;

        // pipe docker attach output into stdout
        while let Some(Ok(output)) = output.next().await {
            stdout.write_all(output.into_bytes().as_ref())?;
            stdout.flush()?;
        }
    }
    Ok(())
}
