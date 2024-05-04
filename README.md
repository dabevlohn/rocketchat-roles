# Create custom views for RocketChat database collections (Mongo) 

We use Rocket :) web framework written in Rust

```shell
curl http://localhost:8000/users/ | jq
```

```json
[
  {
    "_id": "rocket.cat",
    "roles": [
      "bot"
    ],
    "status": "online",
    "username": "rocket.cat",
    "active": true
  },
  {
    "_id": "G7RXKXa3w9pMn3RMt",
    "roles": [
      "user",
      "admin"
    ],
    "status": "offline",
    "username": "dpl",
    "active": true
  }
]
```

```shell
curl http://localhost:8000/permissions/ | jq
```

```json
[
  {
    "_id": "access-permissions",
    "roles": [
      "admin"
    ]
  },
  {
    "_id": "access-marketplace",
    "roles": [
      "admin",
      "user"
    ]
  },
  {
    "_id": "access-setting-permissions",
    "roles": [
      "admin"
    ]
  },
  {
    "_id": "add-oauth-service",
    "roles": [
      "admin"
    ]
  },
  {
    "_id": "add-user-to-joined-room",
    "roles": [
      "admin",
      "owner",
      "moderator"
    ]
  },
  {
    "_id": "add-user-to-any-c-room",
    "roles": [
      "admin"
    ]
  },
  {
    "_id": "kick-user-from-any-c-room",
    "roles": [
      "admin"
    ]
  },
  {
    "_id": "api-bypass-rate-limit",
    "roles": [
      "admin",
      "bot",
      "app"
    ]
  },
  {
    "_id": "archive-room",
    "roles": [
      "admin",
      "owner"
    ]
  },
  {
    "_id": "assign-admin-role",
    "roles": [
      "admin"
    ]
  },
  {
    "_id": "assign-roles",
    "roles": [
      "admin"
    ]
  },
  {
    "_id": "ban-user",
    "roles": [
      "admin",
      "owner",
      "moderator"
    ]
  },
...
```

```shell
curl http://localhost:8000/roles/ | jq
```

```json
[
  {
    "_id": "admin",
    "scope": "Users",
    "name": "admin"
  },
  {
    "_id": "moderator",
    "scope": "Subscriptions",
    "name": "moderator"
  },
  {
    "_id": "leader",
    "scope": "Subscriptions",
    "name": "leader"
  },
  {
    "_id": "owner",
    "scope": "Subscriptions",
    "name": "owner"
  },
  {
    "_id": "user",
    "scope": "Users",
    "name": "user"
  },
  {
    "_id": "bot",
    "scope": "Users",
    "name": "bot"
  },
  {
    "_id": "app",
    "scope": "Users",
    "name": "app"
  },
  {
    "_id": "guest",
    "scope": "Users",
    "name": "guest"
  },
  {
    "_id": "anonymous",
    "scope": "Users",
    "name": "anonymous"
  },
  {
    "_id": "livechat-agent",
    "scope": "Users",
    "name": "livechat-agent"
  },
  {
    "_id": "livechat-manager",
    "scope": "Users",
    "name": "livechat-manager"
  },
  {
    "_id": "livechat-monitor",
    "scope": "Users",
    "name": "livechat-monitor"
  }
]
```
