//! Windows storage device enumeration.
//!
//! Enumerates storage devices and logical volumes on Windows:
//! - Queries logical drives via `sysinfo::Disks` for total and used bytes.
//! - Classifies drives into USB removable vs internal storage.
//! - When unprivileged (running without Administrator rights), gracefully falls
//!   back to logical drive descriptors with placeholders for restricted fields.
//!
//! Note: this file is only compiled on Windows. The cfg gate lives at the
//! module declaration in `collect/mod.rs`.

use sysinfo::{DiskKind, Disks};

/// Representation of a storage device or volume discovered on Windows.
#[derive(Debug, Clone, Default)]
pub struct WindowsDevice {
    pub name: String,
    pub kind: WindowsKind,
    pub model: String,
    pub bus: String,
    pub firmware: Option<String>,
    pub serial: Option<String>,
    pub size_bytes: u64,
    pub used_bytes: u64,
    pub removable: bool,
    pub smart_ok: Option<bool>,
}

/// Hardware drive category on Windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindowsKind {
    Ssd,
    Hdd,
    UsbMassStorage,
    #[default]
    Unknown,
}

/// Enumerate storage devices and attached volumes.
pub fn collect() -> Vec<WindowsDevice> {
    let disks = Disks::new_with_refreshed_list();
    let mut out = Vec::new();

    for d in disks.list() {
        let mount = d.mount_point().to_string_lossy().into_owned();
        let total = d.total_space();
        let avail = d.available_space();
        let used = total.saturating_sub(avail);
        let removable = d.is_removable();

        // Clean name (e.g. "C:" from "C:\\")
        let name = if mount.ends_with('\\') || mount.ends_with('/') {
            mount.trim_end_matches(['\\', '/']).to_string()
        } else {
            mount.clone()
        };

        let kind = if removable {
            WindowsKind::UsbMassStorage
        } else {
            match d.kind() {
                DiskKind::SSD => WindowsKind::Ssd,
                DiskKind::HDD => WindowsKind::Hdd,
                _ => WindowsKind::Unknown,
            }
        };

        let model = if removable {
            format!("Removable Drive ({name})")
        } else {
            format!("Local Disk ({name})")
        };

        out.push(WindowsDevice {
            name,
            kind,
            model,
            bus: String::new(),
            firmware: None,
            serial: None,
            size_bytes: total,
            used_bytes: used,
            removable,
            smart_ok: None,
        });
    }

    out
}
