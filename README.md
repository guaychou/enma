# Enma [WIP]

### What is this ? 

#### My personal project to retrieving data from Newrelic 

#### There are some metrics (prefix --> /newrelic/v1)
- Cpu used core ( path: /cpu-used-core)
- Cpu requested core ( path: /cpu-requested-core)
- Thread Count (path: /thread-count)
- Memory Heap ( path: /memory-heap-used ) --> Java Heap Memory
- Throughput ( path: /throughput ) 
- Total Pods K8s ( path: /pods-total)
- Response time average (path: /response-time-average)

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

```


### Example enma config

```yaml
newrelic:
  api_key: <YOUR_API_KEY_HERE>
  account_id: <YOUR_ACCOUNT_ID_HERE>
server:
  host: 0.0.0.0
  port: 8080
```


### Example logging config

```yaml

# Scan this file for changes every 30 seconds
refresh_rate: 10 seconds

appenders:
  stdout:
    kind: console

root:
  level: info
  appenders:
    - stdout

loggers:
  app::backend::db:
    level: info
```