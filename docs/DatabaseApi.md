# \DatabaseApi

All URIs are relative to *http://localhost:6363/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**db_get**](DatabaseApi.md#db_get) | **GET** /db/ | List details of all databases available to the authorized user.
[**db_organization_database_delete**](DatabaseApi.md#db_organization_database_delete) | **DELETE** /db/{organization}/{database} | Delete a database
[**db_organization_database_get**](DatabaseApi.md#db_organization_database_get) | **GET** /db/{organization}/{database} | List details of the database under the given organization.
[**db_organization_database_head**](DatabaseApi.md#db_organization_database_head) | **HEAD** /db/{organization}/{database} | Check that a db exists
[**db_organization_database_post**](DatabaseApi.md#db_organization_database_post) | **POST** /db/{organization}/{database} | Create a database
[**db_organization_database_put**](DatabaseApi.md#db_organization_database_put) | **PUT** /db/{organization}/{database} | Change qualities of a database (label, comment, etc.)
[**optimize_path_post**](DatabaseApi.md#optimize_path_post) | **POST** /optimize/{path} | 



## db_get

> crate::models::Database db_get(branches, verbose)
List details of all databases available to the authorized user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**branches** | Option<**bool**> | Return branches or not |  |[default to false]
**verbose** | Option<**bool**> | Return all available information |  |[default to false]

### Return type

[**crate::models::Database**](Database.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## db_organization_database_delete

> crate::models::DbOrganizationDatabaseDelete200Response db_organization_database_delete(organization, database, force)
Delete a database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | Organization for the database | [required] |
**database** | **String** | Database name | [required] |
**force** | Option<**bool**> | Force database to be delete (useful for databases in inconsistent states) |  |[default to false]

### Return type

[**crate::models::DbOrganizationDatabaseDelete200Response**](_db__organization___database__delete_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## db_organization_database_get

> Vec<crate::models::Database> db_organization_database_get(organization, database, branches, verbose)
List details of the database under the given organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | Organization for the database | [required] |
**database** | **String** | Database name | [required] |
**branches** | Option<**bool**> | Return branches or not |  |[default to false]
**verbose** | Option<**bool**> | Return all available information |  |[default to false]

### Return type

[**Vec<crate::models::Database>**](Database.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## db_organization_database_head

> serde_json::Value db_organization_database_head(organization, database)
Check that a db exists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | Organization for the database | [required] |
**database** | **String** | Database name | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## db_organization_database_post

> crate::models::DbOrganizationDatabasePost200Response db_organization_database_post(organization, database, db_organization_database_put_request)
Create a database

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | Organization for the database | [required] |
**database** | **String** | Database name | [required] |
**db_organization_database_put_request** | [**DbOrganizationDatabasePutRequest**](DbOrganizationDatabasePutRequest.md) |  | [required] |

### Return type

[**crate::models::DbOrganizationDatabasePost200Response**](_db__organization___database__post_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## db_organization_database_put

> crate::models::DbOrganizationDatabasePut200Response db_organization_database_put(organization, database, db_organization_database_put_request)
Change qualities of a database (label, comment, etc.)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | Organization for the database | [required] |
**database** | **String** | Database name | [required] |
**db_organization_database_put_request** | [**DbOrganizationDatabasePutRequest**](DbOrganizationDatabasePutRequest.md) |  | [required] |

### Return type

[**crate::models::DbOrganizationDatabasePut200Response**](_db__organization___database__put_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## optimize_path_post

> crate::models::OptimizePathPost200Response optimize_path_post(path)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Database Path | [required] |

### Return type

[**crate::models::OptimizePathPost200Response**](_optimize__path__post_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

