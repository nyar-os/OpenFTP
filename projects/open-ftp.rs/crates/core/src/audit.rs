use serde::Serialize;

/// Stable capability ids shared by the native boundary, CLI, skills, and audit.
pub mod capability_ids {
    /// Report the capability table.
    pub const CAPABILITIES: &str = "capabilities";
    /// Report the product and native versions.
    pub const VERSION: &str = "version";
    /// Query local audit records.
    pub const AUDIT_QUERY: &str = "audit-query";
    /// Configure audit actor resolution.
    pub const AUDIT_CONFIGURE: &str = "audit-configure";
    /// Host the WebView GUI.
    pub const GUI: &str = "gui";

    /// Inventory configured FTP and FTPS endpoints.
    pub const ENDPOINT_LIST: &str = "endpoint-list";
    /// Configure control, TLS, data, and passive ports for an endpoint.
    pub const ENDPOINT_PORTS: &str = "endpoint-ports";
    /// Open the control channel, authenticate, and negotiate TLS.
    pub const FTP_CONNECT: &str = "ftp-connect";
    /// List a remote directory with encoding negotiation.
    pub const FTP_LIST: &str = "ftp-list";
    /// Upload or download files with resume and verification.
    pub const FTP_TRANSFER: &str = "ftp-transfer";
    /// Mirror a tree with an explicit delete and conflict policy.
    pub const FTP_MIRROR: &str = "ftp-mirror";
    /// Scan and compute an rsync-style delta plan.
    pub const RSYNC_PLAN: &str = "rsync-plan";
    /// Declare the sync direction and reject plans that disagree.
    pub const SYNC_DIRECTION: &str = "sync-direction";
    /// Resolve conflicts with an explicit policy and confirmation.
    pub const SYNC_CONFLICT_POLICY: &str = "sync-conflict-policy";
    /// Render a machine-readable dry-run preview of a sync plan.
    pub const SYNC_PREVIEW: &str = "sync-preview";
    /// Undo files created or overwritten by one recorded plan.
    pub const SYNC_UNDO: &str = "sync-undo";
    /// Apply a delta plan with a transaction and rollback boundary.
    pub const RSYNC_APPLY: &str = "rsync-apply";
    /// Schedule a sync task with idempotent retries.
    pub const SYNC_SCHEDULE: &str = "sync-schedule";

    /// Install the host integration and desktop entry.
    pub const HOST_INSTALL: &str = "host-install";
    /// Remove the host integration and clean residual state.
    pub const HOST_UNINSTALL: &str = "host-uninstall";
}

/// Audit fields recorded at the native boundary.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditFields {
    /// Stable capability id.
    pub capability: String,
    /// Number of targets in the call.
    pub target_count: usize,
    /// Redacted target summary. Never contains credentials or full remote paths.
    pub target_summary: String,
}

impl AuditFields {
    /// Build redacted audit fields for one capability call.
    pub fn new(capability: impl Into<String>, target_count: usize, target_summary: impl Into<String>) -> Self {
        Self { capability: capability.into(), target_count, target_summary: target_summary.into() }
    }
}
