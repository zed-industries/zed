use super::super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DependabotAlert {
    pub number: i64,
    pub state: State,
    pub dependency: Dependency,
    pub security_advisory: SecurityAdvisory,
    pub security_vulnerability: SecurityVulnerability,
    pub url: Url,
    pub html_url: Url,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dismissed_at: Option<DateTime<Utc>>,
    pub dismissed_by: Option<SimpleUser>,
    pub dismissed_reason: Option<DissmisedReason>,
    pub dismissed_comment: Option<String>,
    pub fixed_at: Option<DateTime<Utc>>,
    pub auto_dismissed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DissmisedReason {
    FixStarted,
    Inaccurate,
    NoBandwidth,
    NotUsed,
    TolerableRisk,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    AutoDismissed,
    Dismissed,
    Fixed,
    Open,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dependency {
    pub package: Package,
    pub manifest_path: String,
    pub scope: Option<Scope>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    Development,
    Runtime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub ecosystem: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityAdvisory {
    pub ghsa_id: String,
    pub cve_id: Option<String>,
    pub summary: String,
    pub description: String,
    pub vulnerabilities: Vec<Vulnerability>,
    pub severity: Severity,
    pub cvss: Cvss,
    pub cvss_severities: CvssSeverities,
    pub cwes: Vec<Cwe>,
    pub identifiers: Vec<Identifier>,
    pub references: Vec<Reference>,
    pub published_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub withdrawn_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vulnerability {
    pub package: Package,
    pub severity: Severity,
    pub vulnerable_version_range: String,
    pub first_patched_version: Option<FirstPatchedVersion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirstPatchedVersion {
    pub identifier: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cvss {
    pub vector_string: Option<String>,
    pub score: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CvssSeverities {
    pub cvss_v3: Option<Cvss>,
    pub cvss_v4: Option<Cvss>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cwe {
    pub cwe_id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub r#type: AdvisoryType,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdvisoryType {
    CVE,
    GHSA,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub url: Url,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityVulnerability {
    pub package: Package,
    pub severity: Severity,
    pub vulnerable_version_range: String,
    pub first_patched_version: Option<FirstPatchedVersion>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDependabotAlert<'a> {
    pub state: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed_reason: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed_comment: Option<&'a str>,
}
