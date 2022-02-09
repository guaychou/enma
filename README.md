# Enma [WIP]

### What is this ? 

#### My personal project to retrieving data from Newrelic 

#### There are some metrics (prefix --> /v1/newrelic)
- Cpu used core ( path: /cpu-used-core)
- Cpu requested core ( path: /cpu-requested-core)
- Thread Count (path: /thread-count)
- Memory Heap ( path: /memory-heap-used ) --> Java Heap Memory
- Throughput ( path: /throughput ) 
- Total Pods K8s ( path: /pods-total)
- Response time average (path: /response-time-average)


### Example Vault configuration
```json
{
  "newrelic": {
    "account_id": 00000,
    "api_key": "xxxxx"
  },
  "server": {
    "buffer": 10,
    "concurrency_limit": 100,
    "limiter_timeout": "10s",
    "port": 8080,
    "rate_limit": 100,
    "timeout": "10s",
    "metrics" : false
  }
}

```

### Request Body
```yaml

{
    "data": {
        "application_name": "test-app",
        "start_time" : "1 minute ago",
        "end_time" : "now"
    }
}

```

### Response Body
```yaml
{
    "api_version": "v0",
    "data": {
        "result": 0.053074583
    }
}

