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
pub struct DbOrganizationDatabasePut404Response {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "api:error", skip_serializing_if = "Option::is_none")]
    pub api_colon_error: Option<Box<crate::models::DbOrganizationDatabasePut404ResponseApiError>>,
    #[serde(rename = "api:message", skip_serializing_if = "Option::is_none")]
    pub api_colon_message: Option<String>,
    #[serde(rename = "api:status", skip_serializing_if = "Option::is_none")]
    pub api_colon_status: Option<String>,
}

impl DbOrganizationDatabasePut404Response {
    pub fn new() -> DbOrganizationDatabasePut404Response {
        DbOrganizationDatabasePut404Response {
            at_type: None,
            api_colon_error: None,
            api_colon_message: None,
            api_colon_status: None,
        }
    }
}

