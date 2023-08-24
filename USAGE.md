# Rusty Submaraine

This is not a real application. It is not fit for use of any kind. This document is a reminder to myself on how to call the application.

## Routes

### Hello (index)

Route: `GET /`
Headers: _None_


```bash
curl http://localhost:8000/
```

### Goodbye

Route: `GET /goodbye`
Headers: _None_

```bash
curl http://localhost:8000/goodbye
```

### Delay

Route: `GET /delay/<seconds>`
Headers: _None_

```bash
curl http://localhost:8000/delay/4
```

### Typical Create

Route: `POST /typical-create`
Headers: `X-Api-Key: <uuid>`
Returns: _JSON_ -> `{"id":"<uuid>"}`

```bash
curl -X POST \
     -H "X-Api-Key: $(cat /proc/sys/kernel/random/uuid)" \
     "http://localhost:8000/typical-create"
```

### Find User

Route: `GET /find-user?<username>`
Headers: _None_
Returns: _JSON_ -> Maybe a user record

```bash
curl "http://localhost:8000/find-user?username=codeman99"
```

### Create User

Route: `POST /create-user`
Headers: `X-Api-Key: <uuid>`
Body: `{"id":0,"username":"<username>","first":"<first>","last":"<last>","age":<age>}`
Returns: _Text_ -> `Success`

```bash
curl -H "X-Api-Key: $(cat /proc/sys/kernel/random/uuid)" \
     --data '{"id":0,"username":"codeman99"}'
     "http://localhost:8000/create-user"
```
