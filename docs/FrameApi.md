# \FrameApi

All URIs are relative to *http://localhost:6363/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**schema_get**](FrameApi.md#schema_get) | **GET** /schema | Get the full class frame for a class or all classes for a schema



## schema_get

> serde_json::Value schema_get(path, compress_ids, expand_abstract)
Get the full class frame for a class or all classes for a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Path for document | [required] |
**compress_ids** | Option<**bool**> | Compress the URLs returned using prefixes |  |[default to true]
**expand_abstract** | Option<**bool**> | Whether to expand abstract classes into lists of concrete classes in frame options |  |[default to true]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

