# \ChangeManagementApi

All URIs are relative to *http://localhost:6363/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**diff_post**](ChangeManagementApi.md#diff_post) | **POST** /diff | 
[**patch_post**](ChangeManagementApi.md#patch_post) | **POST** /patch | 



## diff_post

> serde_json::Value diff_post(diff_post_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**diff_post_request** | [**DiffPostRequest**](DiffPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_post

> serde_json::Value patch_post(patch_post_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**patch_post_request** | [**PatchPostRequest**](PatchPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

