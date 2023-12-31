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
pub struct Database {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "creation_date", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            at_id: None,
            at_type: None,
            comment: None,
            creation_date: None,
            label: None,
            name: None,
            state: None,
        }
    }
}


