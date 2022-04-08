## Commandd (clients)
There are certain commands that we can run on the client side, you can check this information below


### Ping Client
To measure websocket latency we need to send a message to the API and then it returns back to you to time it.

**Requirements to run command**:
 - [X] It is mandatory that the client is registered in the API.
 - [ ] It is mandatory to send metadata.
 - [ ] It is necessary to send identification tracking in the data
 - [ ] It is necessary to put session ID in the metadata.
 - [ ] Must run on Shard Master

### **Client**
When sending this metadata you need to store the response time until the API responds.
```json
{
    "type": 3
}
```

### **API**:
These are metadata that the API will return to the Client
```json
{
  "type": 3,
  "service": "gateway",
  "event": "GATEWAY_PING",
  "data": {}
}
```


### Register Client
This is a very important point for the API, which is to identify which client we are connected to to use the public key and with the identification that you entered in `.env`. By registering you will have a simple release of commands in some parts of the API.

#### **Requirements to run command**:
- [X] It is mandatory that the client is registered in the API.
- [x] It is mandatory to send metadata.
- [x] It is necessary to send identification tracking in the data
- [ ] It is necessary to put session ID in the metadata.
- [x] Must run on Shard Master

### **Client**
When sending this metadata you need to store the response time until the API responds.
```json
{
    "type": 1,
    "d": {
      "public_key": "Key publishes that Discord Application release to your bot.",
      "shard_in": 0, // The position of the shard.
      "flags": 0     // Flag to release some resources for Shard development.
    }
    
}
```

### **API**:
These are metadata that the API will return to the Client
```json
{
  "data": {
    "application_bot": {
      "flags": 1,
      "id": "{id-bot}",
      "public_key": "[REDACTED]"
    },
    "session_id": "BYNZETQfmpZcPQiEKZDp7RZpLGv2EJHk4nCMJtN7ci08VDpGK8ywINuc2BkYPCCQn7SZPpjHfWYuCyECN7kC9IUkbiHz6Q9Vat6OW6yamLc0X59zBSCIjhtu==",
    "shards_config": [
      [
        [
          [0, 1]
        ]
      ]
    ],
    "shards_stats": [
      [
        [
          [
            [
              [
                [
                  {
                    "development": false,
                    "disconnected": false,
                    "receibe_ping": 0,
                    "received": 0,
                    "send_ping": 0,
                    "sending": 0,
                    "shard_hash": "C2iIDUxoUEVEh4IjUN5eQpx87UKcsqsEQfs",
                    "shard_id": 0
                  }
                ]
              ]
            ]
          ]
        ]
      ]
    ]
  },
  "event": "GATEWAY_READY", // This can return in various event types.
  "service": "gateway",
  "type": 1
}
```
