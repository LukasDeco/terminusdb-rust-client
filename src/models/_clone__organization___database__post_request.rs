/*
 * TerminusDB API
 *
 * API for TerminusDB
 *
 * The version of the OpenAPI document: 10.0.3
 * Contact: team@terminusdb.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloneOrganizationDatabasePostRequest {
    /// The comment to use in the cloned database
    #[serde(rename = "comment")]
    pub comment: String,
    /// The label to use for the cloned database
    #[serde(rename = "label")]
    pub label: String,
    /// The URL of the database to clone
    #[serde(rename = "remote_url")]
    pub remote_url: String,
    /// Whether to make the newly cloned database public
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
}

impl CloneOrganizationDatabasePostRequest {
    pub fn new(comment: String, label: String, remote_url: String) -> CloneOrganizationDatabasePostRequest {
        CloneOrganizationDatabasePostRequest {
            comment,
            label,
            remote_url,
            public: None,
        }
    }
}


