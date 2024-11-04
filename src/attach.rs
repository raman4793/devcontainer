use bollard::Docker;

use crate::config::DevContainerSpec;
use crate::utils::exec;

#[cfg(not(windows))]
pub(crate) async fn attach(dev_container_spec: &DevContainerSpec, client: &Docker) {
    exec(dev_container_spec, client, vec!["sh"]).await;
}

