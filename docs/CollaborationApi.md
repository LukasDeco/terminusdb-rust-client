# \CollaborationApi

All URIs are relative to *http://localhost:6363/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clone_organization_database_post**](CollaborationApi.md#clone_organization_database_post) | **POST** /clone/{organization}/{database} | 
[**fetch_path_post**](CollaborationApi.md#fetch_path_post) | **POST** /fetch/{path} | Fetch the remote repository for the given path (default is origin)
[**pull_path_post**](CollaborationApi.md#pull_path_post) | **POST** /pull/{path} | Pull to a branch from a remote
[**push_path_post**](CollaborationApi.md#push_path_post) | **POST** /push/{path} | Push the branch to the remote



## clone_organization_database_post

> serde_json::Value clone_organization_database_post(organization, database, clone_organization_database_post_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | Organization for the database | [required] |
**database** | **String** | Database name | [required] |
**clone_organization_database_post_request** | [**CloneOrganizationDatabasePostRequest**](CloneOrganizationDatabasePostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_path_post

> crate::models::FetchPathPost200Response fetch_path_post(path, authorization_remote)
Fetch the remote repository for the given path (default is origin)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Path for branch | [required] |
**authorization_remote** | **String** | The authorization to use on the remote | [required] |

### Return type

[**crate::models::FetchPathPost200Response**](_fetch__path__post_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_path_post

> crate::models::PullPathPost200Response pull_path_post(path, pull_path_post_request)
Pull to a branch from a remote

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Path for branch | [required] |
**pull_path_post_request** | [**PullPathPostRequest**](PullPathPostRequest.md) |  | [required] |

### Return type

[**crate::models::PullPathPost200Response**](_pull__path__post_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## push_path_post

> crate::models::PushPathPost200Response push_path_post(path, push_path_post_request)
Push the branch to the remote

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Path for branch | [required] |
**push_path_post_request** | [**PushPathPostRequest**](PushPathPostRequest.md) |  | [required] |

### Return type

[**crate::models::PushPathPost200Response**](_push__path__post_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

