use crate::{audit::capability_ids, errors::CapabilityState};
use serde::Serialize;

/// The host platform reported by the native crate.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformInfo {
    /// Operating system constant such as `windows`.
    pub os: &'static str,
    /// Architecture constant such as `x86_64`.
    pub arch: &'static str,
}

/// One row in the capability matrix.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityItem {
    /// Stable capability id.
    pub id: &'static str,
    /// `null` when the capability is available, otherwise one of the three states.
    pub state: Option<CapabilityState>,
    /// Stable error code matching `state`. `null` when available.
    pub code: Option<&'static str>,
    /// Why the capability is unavailable.
    pub reason: &'static str,
    /// What the capability does once implemented.
    pub detail: &'static str,
}

impl CapabilityItem {
    /// Whether the capability can be invoked on this host right now.
    pub const fn is_available(&self) -> bool {
        self.state.is_none()
    }
}

/// The full capability report returned to CLI, GUI, and agents.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilitiesReport {
    /// Product name.
    pub app: &'static str,
    /// Current host platform.
    pub platform: PlatformInfo,
    /// Native package version.
    pub version: &'static str,
    /// Every registered capability, including unavailable rows.
    pub items: Vec<CapabilityItem>,
    /// Contract notes that apply to the whole table.
    pub notes: Vec<&'static str>,
}

impl CapabilitiesReport {
    /// Look up a capability row by its stable id.
    pub fn get(&self, id: &str) -> Option<&CapabilityItem> {
        self.items.iter().find(|item| item.id == id)
    }
}

const SKELETON_REASON: &str = "The skeleton round registers the capability but does not implement it.";

const DOMAIN: &[(&str, &str)] = &[
    (capability_ids::ENDPOINT_LIST, "Inventory configured FTP and FTPS endpoints with redacted connection metadata."),
    (capability_ids::FTP_CONNECT, "Open the control channel, authenticate, and negotiate explicit or implicit TLS."),
    (capability_ids::FTP_LIST, "List a remote directory with encoding negotiation and a deterministic order."),
    (capability_ids::FTP_TRANSFER, "Upload or download files with resume, checksum verification, and progress."),
    (capability_ids::FTP_MIRROR, "Mirror a tree with an explicit delete policy and conflict confirmation."),
    (capability_ids::RSYNC_PLAN, "Scan both sides and preview an rsync-style delta plan without writing."),
    (capability_ids::RSYNC_APPLY, "Apply a delta plan with a transaction and rollback boundary."),
    (capability_ids::SYNC_SCHEDULE, "Schedule a sync task with idempotent retries and stable run ids."),
    (capability_ids::HOST_INSTALL, "Install the host integration and desktop entry."),
    (capability_ids::HOST_UNINSTALL, "Remove the host integration and clean residual state."),
];

const META: &[(&str, &str)] = &[
    (capability_ids::CAPABILITIES, "Report the capability matrix."),
    (capability_ids::VERSION, "Report product and native versions."),
    (capability_ids::AUDIT_QUERY, "Query local audit records."),
    (capability_ids::AUDIT_CONFIGURE, "Resolve and configure the audit actor."),
];

/// Current host platform.
pub const fn platform() -> PlatformInfo {
    PlatformInfo { os: std::env::consts::OS, arch: std::env::consts::ARCH }
}

/// Build the capability report. Domain rows stay unavailable in the skeleton round.
pub fn capabilities() -> CapabilitiesReport {
    let mut items = Vec::with_capacity(DOMAIN.len() + META.len());
    for (id, detail) in META {
        items.push(CapabilityItem { id, state: None, code: None, reason: "available", detail });
    }
    for (id, detail) in DOMAIN {
        items.push(CapabilityItem {
            id,
            state: Some(CapabilityState::NotImplemented),
            code: Some("not_implemented"),
            reason: SKELETON_REASON,
            detail,
        });
    }
    CapabilitiesReport {
        app: "open-ftp",
        platform: platform(),
        version: env!("CARGO_PKG_VERSION"),
        items,
        notes: vec![
            "Domain capabilities are intentionally not_implemented in this skeleton round.",
            "Capability, version, audit, and GUI meta capabilities are available.",
            "FTPS uses the product-owned TLS stack. No capability may fall back to an external ftp, sftp, rsync, or openssl process.",
        ],
    }
}
