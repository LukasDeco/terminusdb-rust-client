# \BranchesApi

All URIs are relative to *http://localhost:6363/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**branch_path_delete**](BranchesApi.md#branch_path_delete) | **DELETE** /branch/{path} | 
[**branch_path_post**](BranchesApi.md#branch_path_post) | **POST** /branch/{path} | 
[**reset_path_post**](BranchesApi.md#reset_path_post) | **POST** /reset/{path} | 
[**squash_path_get**](BranchesApi.md#squash_path_get) | **GET** /squash/{path} | 



## branch_path_delete

> crate::models::BranchPathPost200Response branch_path_delete(path)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Path for branch | [required] |

### Return type

[**crate::models::BranchPathPost200Response**](_branch__path__post_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## branch_path_post

> crate::models::BranchPathPost200Response branch_path_post(path, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Path for branch | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::BranchPathPost200Response**](_branch__path__post_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_path_post

> crate::models::ResetPathPost200Response reset_path_post(path, reset_path_post_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Path for branch | [required] |
**reset_path_post_request** | [**ResetPathPostRequest**](ResetPathPostRequest.md) |  | [required] |

### Return type

[**crate::models::ResetPathPost200Response**](_reset__path__post_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## squash_path_get

> crate::models::SquashPathGet200Response squash_path_get(path)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Path for branch | [required] |

### Return type

[**crate::models::SquashPathGet200Response**](_squash__path__get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

