# \WoqlApi

All URIs are relative to *http://localhost:6363/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**woql_path_post**](WoqlApi.md#woql_path_post) | **POST** /woql/{path} | 
[**woql_post**](WoqlApi.md#woql_post) | **POST** /woql | 



## woql_path_post

> woql_path_post(path, woql_query)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Database Path | [required] |
**woql_query** | [**WoqlQuery**](WoqlQuery.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## woql_post

> woql_post(woql_query)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**woql_query** | [**WoqlQuery**](WoqlQuery.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

