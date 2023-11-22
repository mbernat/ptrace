use nix::{errno::Errno, sys::ptrace, unistd::Pid};
use sysinfo::{PidExt, ProcessExt, RefreshKind, System, SystemExt};
use tracing::info;

#[derive(Debug, thiserror::Error)]
enum HostError {
    #[error("Process not found {0}")]
    ProcessNotFound(String),
    #[error("Nix error {0}")]
    NixError(#[from] Errno),
}

type HostResult<T> = Result<T, HostError>;

fn main() -> HostResult<()> {
    let format = tracing_subscriber::fmt::format().pretty();
    tracing_subscriber::fmt().event_format(format).init();

    info!("Starting host");
    let mut sys = System::new_with_specifics(RefreshKind::everything().without_users_list());
    sys.refresh_all();

    let process_name = "guest";
    let process = sys
        .processes_by_name(process_name)
        .next()
        .ok_or_else(|| HostError::ProcessNotFound(process_name.to_string()))?;
    let pid = process.pid().as_u32();
    let pid = Pid::from_raw(pid as i32);
    info!("Found guest with pid {pid}");

    ptrace::attach(pid)?;
    info!("Attached to pid {pid}");

    let user_registers = ptrace::getregs(pid)?;
    info!("{user_registers:#x?}");

    ptrace::detach(pid, Some(nix::sys::signal::Signal::SIGCONT))?;
    info!("Detached from pid {pid}");

    Ok(())
}
