use dnf5daemon::DnfDaemon;
use futures_util::StreamExt;
use log::debug;

pub async fn signal_download_progress(dnf_daemon: &DnfDaemon) -> Result<(), zbus::Error> {
    let mut download_progress = dnf_daemon.base.receive_download_progress().await?;
    while let Some(signal) = download_progress.next().await {
        let args = signal.args()?;
        debug!("\rSignal: download_progress : {:?}", args);
    }
    Ok::<(), zbus::Error>(())
}
pub async fn signal_download_add_new(dnf_daemon: &DnfDaemon) -> Result<(), zbus::Error> {
    let mut download_add_new = dnf_daemon.base.receive_download_add_new().await?;
    while let Some(signal) = download_add_new.next().await {
        let args = signal.args()?;
        debug!("Signal: download_add_new : {:?}", args);
    }
    Ok::<(), zbus::Error>(())
}
