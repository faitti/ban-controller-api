# Api documentation

## **User control**
| Endpoint | Method | Description |
| --- | --- | --- |
| /key | GET | Request apikey from server
| /key | POST | Register new server

### **GET** /key
<details>
    <summary>Headers</summary>

    Content-Type: application/json
</details>

<details>
    <summary>Body</summary>

| Param |  Type | Required | Description |
| --- | --- | --- | --- |
| server | string | Yes | Server name |
| password | string | Yes | Length must be between 8 and 64 characters |
</details>

**Example request**
```curl
curl --request GET 'http://localhost:8080/key' \
--header 'Content-Type: application/json' \
--data-raw '{
    "server": "server_name",
    "password": "password"
}'
```

**Example response**
```JSON
{
    "apikey": "apikey"
}
```

----

### **POST** /key
<details>
    <summary>Headers</summary>

    Content-Type: application/json
</details>

<details>
    <summary>Body</summary>

| Param |  Type | Required | Description |
| --- | --- | --- | --- |
| server | string | Yes | Server name |
| password | string | Yes | Length must be between 8 and 64 characters |
</details>

**Example request**
```curl
curl --request POST 'http://localhost:8080/key' \
--header 'Content-Type: application/json' \
--data-raw '{
    "server": "server_name",
    "password": "password"
}'
```

**Example response**
```JSON
{
    "apikey": "apikey"
}
```