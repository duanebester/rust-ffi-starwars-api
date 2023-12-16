# \DefaultApi

All URIs are relative to *https://swapi.dev/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_character_by_id**](DefaultApi.md#get_character_by_id) | **GET** /people/{id} | Find character by ID



## get_character_by_id

> crate::models::Character get_character_by_id(id)
Find character by ID

Returns a single character

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | ID of character to return | [required] |

### Return type

[**crate::models::Character**](Character.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

