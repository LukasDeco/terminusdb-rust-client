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
pub struct WoqlQuery {
    /// WOQL Query
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<serde_json::Value>,
    #[serde(rename = "commit_info", skip_serializing_if = "Option::is_none")]
    pub commit_info: Option<Box<crate::models::CommitInfo>>,
    /// Check for all errors
    #[serde(rename = "all_witnesses", skip_serializing_if = "Option::is_none")]
    pub all_witnesses: Option<bool>,
}

impl WoqlQuery {
    pub fn new() -> WoqlQuery {
        WoqlQuery {
            query: None,
            commit_info: None,
            all_witnesses: None,
        }
    }
}


