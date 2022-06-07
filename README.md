# Web-CoreUtils

## getput

### API Specification 1.0
All request headers are ignored  

#### `GET /<KEY>`
If success: `200 OK <VALUE>`  
If not found: `404 Not Found`  
If key is too large: `414 URI too long`  
Otherwise: `500 Internal Server Error`  

#### `PUT /<KEY> <BODY=VALUE>`
If key already existed: `200 OK`  
If key did not exist: `201 Created`  
If key is too large: `414 URI too long`  
If value is too large: `413 Payload Too Large`  
Otherwise: `500 Internal Server Error`  

### License

MIT
