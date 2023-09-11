# \DocumentApi

All URIs are relative to *http://localhost:6363/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**document_path_delete**](DocumentApi.md#document_path_delete) | **DELETE** /document/{path} | Delete one or multiple documents
[**document_path_get**](DocumentApi.md#document_path_get) | **GET** /document/{path} | Get a document
[**document_path_post**](DocumentApi.md#document_path_post) | **POST** /document/{path} | Insert a new document
[**document_path_put**](DocumentApi.md#document_path_put) | **PUT** /document/{path} | Replace a document



## document_path_delete

> document_path_delete(path, author, message, graph_type, nuke, id)
Delete one or multiple documents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Path for document | [required] |
**author** | **String** | Author of commit | [required] |
**message** | **String** | Commit message | [required] |
**graph_type** | Option<**String**> | Type of the graph |  |[default to instance]
**nuke** | Option<**bool**> | Totally nuke the document(s) |  |[default to false]
**id** | Option<**String**> | ID of document(s) |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## document_path_get

> serde_json::Value document_path_get(path, graph_type, skip, count, minimized, as_list, unfold, id, r#type, compress_ids)
Get a document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Path for document | [required] |
**graph_type** | Option<**String**> | Type of the graph |  |[default to instance]
**skip** | Option<**i32**> | Skip a certain amount of documents |  |[default to 0]
**count** | Option<**i32**> | Number of entries to show |  |
**minimized** | Option<**bool**> | Minify the output |  |[default to true]
**as_list** | Option<**bool**> | Return the JSONs as list instead of concatenated json |  |[default to false]
**unfold** | Option<**bool**> | Unfold the documents (join other referenced documents) |  |[default to true]
**id** | Option<**String**> | Specific document ID to look for |  |
**r#type** | Option<**String**> | Get documents of only a specific type |  |
**compress_ids** | Option<**bool**> | Whether to compress all ids using prefixes |  |[default to true]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## document_path_post

> serde_json::Value document_path_post(path, author, message, graph_type, full_replace, raw_json)
Insert a new document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Path for document | [required] |
**author** | **String** | Author of commit | [required] |
**message** | **String** | Commit message | [required] |
**graph_type** | Option<**String**> | Type of the graph |  |[default to instance]
**full_replace** | Option<**bool**> | Fully replace a document |  |[default to false]
**raw_json** | Option<**bool**> | Whether to interpret the posted document as arbitrary untyped JSON |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## document_path_put

> document_path_put(path, author, message, graph_type, create, raw_json)
Replace a document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | Path for document | [required] |
**author** | **String** | Author of commit | [required] |
**message** | **String** | Commit message | [required] |
**graph_type** | Option<**String**> | Type of the graph |  |[default to instance]
**create** | Option<**bool**> | Create a document if it does not exist |  |[default to false]
**raw_json** | Option<**bool**> | Treat incoming document as raw json (not type checked) |  |[default to false]

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

