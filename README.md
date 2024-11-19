# seki

seki is a CLI tool to help parse and aggregate access log data for further analysis. The tool is intended to be used with nginx access logs (JSON format), piped out to tools like `jq`.

The name seki comes from the Japanese terms 積 (from 集積 which means accumulate) and 析 (from 分析 which means analysis). These kanji are both read "seki".

## Installation

TODO

## Example

Let's say you have an access log file like this:

```sh
$ cat access.log
{"time":"19/Nov/2024:09:18:34 +0900","status":"200","method":"GET","uri":"/foobar","response_time":"0.476"}
{"time":"19/Nov/2024:09:18:58 +0900","status":"400","method":"GET","uri":"/","response_time":"0.239"}
{"time":"19/Nov/2024:09:19:22 +0900","status":"302","method":"GET","uri":"/hello","response_time":"0.113"}
{"time":"19/Nov/2024:09:20:12 +0900","status":"200","method":"GET","uri":"/foobar","response_time":"0.732"}
```

seki aggregates the logs into one simple JSON.

```sh
$ cat access.log | seki
[
    {
        "uri": "/foobar",
        "method": "GET",
        "count": 2,
        "status_code": {
            "1xx": 0,
            "2xx": 2,
            "3xx": 0,
            "4xx": 0,
            "5xx": 0
        },
        "response_time": {
            "min": 0.476,
            "max": 0.732,
            "sum": 1.208,
            "avg": 0.604,
            "p99": 0.732
        }
    },
    {
        "uri": "/",
        "method": "GET",
        "count": 1,
        "status_code": {
            "1xx": 0,
            "2xx": 0,
            "3xx": 0,
            "4xx": 1,
            "5xx": 0
        },
        "response_time": {
            "min": 0.239,
            "max": 0.239,
            "sum": 0.239,
            "avg": 0.239,
            "p99": 0.239
        }
    },
    {
        "uri": "/hello",
        "method": "GET",
        "count": 1,
        "status_code": {
            "1xx": 0,
            "2xx": 0,
            "3xx": 1,
            "4xx": 0,
            "5xx": 0
        },
        "response_time": {
            "min": 0.113,
            "max": 0.113,
            "sum": 0.113,
            "avg": 0.113,
            "p99": 0.113
        }
    }
]
```

This can now be used in conjunction with jq. The following sample demonstrates sorting the `uri`s by `response_time.sum`.

```sh
$ cat access.log | seki | jq 'sort_by(-.response_time.sum) | .[] | {uri: .uri, response_time_sum: .response_time.sum}'
{
  "uri": "/foobar",
  "response_time_sum": 1.208
}
{
  "uri": "/",
  "response_time_sum": 0.239
}
{
  "uri": "/hello",
  "response_time_sum": 0.113
}
```
