# Api documentation

## **Server management**
| Endpoint | Method | Description |
| --- | --- | --- |
| /key | GET | Request server's apikey, sort of a login route
| /key | POST | Register new server
| /key | PATCH | Generate new apikey

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

---

### **PATCH** /key
<details>
    <summary>Headers</summary>

    Bearer: apikey
</details>

**Example request**
```curl
curl --request PATCH 'http://localhost:8080/key \
--header 'Bearer: apikey'
```

**Example response**
```JSON
{
    "apikey": "new_apikey"
}
```