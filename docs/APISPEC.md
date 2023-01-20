# Api documentation

## **General info**
Error response:
```JSON
{
    "error": "Unauthorized"
}
```

Status code will always be correct.

---
<br>

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
    Content-Type: application/json
</details>

**Example request**
```curl
curl --request PATCH 'http://localhost:8080/key \
--header 'Bearer: apikey' \
--header 'Content-Type: application/json'
```

**Example response**
```JSON
{
    "apikey": "new_apikey"
}
```

---
<br>

## **Ban management**
| Endpoint | Method | Description |
| --- | --- | --- |
| /ban | POST | Inserts ban data in to the database, for other servers to use that specific ban. |
| /ban | GET | Check if player has been banned. |

### **POST** /ban
<details>
    <summary>Notes</summary>

    Server must be verified by an admin in order to use this route
</details>
<details>
    <summary>Headers</summary>

    Bearer: apikey
    Content-Type: application/json
</details>
<details>
    <summary>Body</summary>

| Param |  Type | Required | Description |
| --- | --- | --- | --- |
| identifiers | json object | Yes | `steam`, `license`, `discord`, `xbox`, `live`, `license2` |
| reason | string | Yes | Ban reason |
| length | number | Yes | Length of the ban in seconds |
</details>

**Example request**
```curl
curl --request POST 'http://localhost:8080/ban \
--header 'Bearer: apikey' \
--header 'Content-Type: application/json' \
--data-raw '{
    "identifiers": {
        "steam": "x",
        "discord": "x",
        "license": "x",
        "license2": "x",
        "live": "x",
        "xbox": "x"
    },
    "reason": "cheater",
    "length": 1500000
}'
```

**Example response**
```JSON
{
    "message": "Successfully banned player"
}
```

---

### **GET** /ban
<details>
    <summary>Notes</summary>

    Json payload must contain at least one (1) identifier!
</details>
<details>
    <summary>Headers</summary>

    Bearer: apikey
    Content-Type: application/json
</details>
<details>
    <summary>Body</summary>

| Param |  Type | Required | Description |
| --- | --- | --- | --- |
| steam | string | No | Hex ID |
| license | string | No | Rockstar license |
| license2 | string | No | Some license, dunno |
| discord | string | No | Discord ID |
| xbox | string | No | Xbox identifier |
| live | string | No | Live identifier |
</details>

**Example request**
```curl
curl --request POST 'http://localhost:8080/ban \
--header 'Bearer: apikey' \
--header 'Content-Type: application/json' \
--data-raw '{
    "steam": "x",
    "discord": "x",
    "license": "x",
    "live": "x",
    "license2": "x",
    "xbox": "x"
}'
```

**Example response**
```JSON
{
    "reason": "Cheater",
    "server": "Example roleplay",
    "expires": 1675451426
}