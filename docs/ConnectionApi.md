# \ConnectionApi

All URIs are relative to *http://localhost:6363/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**info_get**](ConnectionApi.md#info_get) | **GET** /info | Get information about the TerminusDB version
[**ok_get**](ConnectionApi.md#ok_get) | **GET** /ok | Simple status update
[**root_get**](ConnectionApi.md#root_get) | **GET** / | Get a list of databases for the authenticated user



## info_get

> crate::models::InfoGet200Response info_get()
Get information about the TerminusDB version

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InfoGet200Response**](_info_get_200_response.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ok_get

> ok_get()
Simple status update

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## root_get

> Vec<crate::models::Database> root_get()
Get a list of databases for the authenticated user

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Database>**](Database.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

