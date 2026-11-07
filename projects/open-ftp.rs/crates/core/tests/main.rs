use open_ftp_core::{
    CapabilityState, Error, ErrorCode,
    audit::{AuditFields, capability_ids},
    platform::{capabilities, platform},
};

const DOMAIN: [&str; 10] = [
    capability_ids::ENDPOINT_LIST,
    capability_ids::FTP_CONNECT,
    capability_ids::FTP_LIST,
    capability_ids::FTP_TRANSFER,
    capability_ids::FTP_MIRROR,
    capability_ids::RSYNC_PLAN,
    capability_ids::RSYNC_APPLY,
    capability_ids::SYNC_SCHEDULE,
    capability_ids::HOST_INSTALL,
    capability_ids::HOST_UNINSTALL,
];

const META: [&str; 4] = [capability_ids::CAPABILITIES, capability_ids::VERSION, capability_ids::AUDIT_QUERY, capability_ids::AUDIT_CONFIGURE];

#[test]
fn every_domain_capability_is_registered_as_not_implemented() {
    let report = capabilities();
    for id in DOMAIN {
        let item = report.get(id).unwrap_or_else(|| panic!("missing capability row: {id}"));
        assert_eq!(item.state, Some(CapabilityState::NotImplemented), "{id} must stay not_implemented");
        assert_eq!(item.code, Some("not_implemented"), "{id} must carry the stable code");
        assert!(!item.is_available(), "{id} must not report as available");
        assert!(!item.reason.is_empty(), "{id} must explain why it is unavailable");
        assert!(!item.detail.is_empty(), "{id} must describe the target behaviour");
    }
}

#[test]
fn sync_capabilities_are_split_into_plan_and_apply() {
    let report = capabilities();
    for id in [capability_ids::RSYNC_PLAN, capability_ids::RSYNC_APPLY] {
        let item = report.get(id).unwrap_or_else(|| panic!("missing capability row: {id}"));
        assert_eq!(item.state, Some(CapabilityState::NotImplemented));
    }
    assert!(report.get(capability_ids::RSYNC_PLAN).expect("plan row").detail.contains("without writing"));
}

#[test]
fn meta_capabilities_are_available() {
    let report = capabilities();
    for id in META {
        let item = report.get(id).unwrap_or_else(|| panic!("missing capability row: {id}"));
        assert!(item.is_available(), "{id} must be available in the skeleton round");
        assert_eq!(item.state, None);
        assert_eq!(item.code, None);
    }
}

#[test]
fn capability_ids_are_unique() {
    let report = capabilities();
    let mut seen = Vec::with_capacity(report.items.len());
    for item in &report.items {
        assert!(!seen.contains(&item.id), "duplicate capability id: {}", item.id);
        seen.push(item.id);
    }
    assert_eq!(report.app, "open-ftp");
    assert_eq!(report.items.len(), DOMAIN.len() + META.len());
    assert!(report.notes.iter().any(|note| note.contains("no capability may fall back") || note.contains("No capability may fall back")));
}

#[test]
fn capability_state_codes_round_trip() {
    for state in [CapabilityState::UnsupportedPlatform, CapabilityState::NotImplemented, CapabilityState::DependencyBlocked] {
        assert_eq!(CapabilityState::from_code(state.code()), Some(state));
    }
    assert_eq!(CapabilityState::from_code("available"), None);
    assert_eq!(ErrorCode::from_state(CapabilityState::NotImplemented).as_str(), "not_implemented");
    assert_eq!(ErrorCode::InvalidArgument.as_str(), "invalid_argument");
    assert_eq!(ErrorCode::Io.as_str(), "io_error");
}

#[test]
fn unavailable_errors_expose_the_capability_and_code() {
    let error = Error::unavailable(CapabilityState::NotImplemented, capability_ids::FTP_TRANSFER, "skeleton round");
    assert_eq!(error.code(), ErrorCode::NotImplemented);
    assert_eq!(error.capability(), Some(capability_ids::FTP_TRANSFER));
    assert_eq!(error.message(), "skeleton round");
    assert_eq!(error.to_string(), "not_implemented (ftp-transfer): skeleton round");
}

#[test]
fn audit_fields_are_redacted_and_typed() {
    let fields = AuditFields::new(capability_ids::FTP_TRANSFER, 2, "endpoint aliases only");
    let value = serde_json::to_value(&fields).expect("audit fields serialize");
    assert_eq!(value["capability"], capability_ids::FTP_TRANSFER);
    assert_eq!(value["targetCount"], 2);
    assert_eq!(value["targetSummary"], "endpoint aliases only");
}

#[test]
fn platform_reports_the_build_host() {
    let info = platform();
    assert!(!info.os.is_empty());
    assert!(!info.arch.is_empty());
}
