# seki

seki is a CLI tool to help parse and aggregate access log data for further analysis. The tool is intended to be used with nginx access logs (JSON format), piped out to tools like `jq`.

The name seki comes from the Japanese terms 積 (from 蓄積 which means accumulate) and 析 (from 分析 which means analysis). These kanji are both read "seki".

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
    "method": "GET",
    "uri": "/hello",
    "count": 1,
    "status_code": {
      "status_1xx": 0,
      "status_2xx": 0,
      "status_3xx": 1,
      "status_4xx": 0,
      "status_5xx": 0
    },
    "response_time": {
      "min": 0.0,
      "max": 0.113,
      "avg": 0.113,
      "sum": 0.113,
      "p50": 0.113,
      "p75": 0.113,
      "p90": 0.113,
      "p95": 0.113,
      "p99": 0.113
    }
  },
  {
    "method": "GET",
    "uri": "/foobar",
    "count": 2,
    "status_code": {
      "status_1xx": 0,
      "status_2xx": 2,
      "status_3xx": 0,
      "status_4xx": 0,
      "status_5xx": 0
    },
    "response_time": {
      "min": 0.0,
      "max": 0.732,
      "avg": 0.604,
      "sum": 1.208,
      "p50": 0.732,
      "p75": 0.732,
      "p90": 0.732,
      "p95": 0.732,
      "p99": 0.732
    }
  },
  {
    "method": "GET",
    "uri": "/",
    "count": 1,
    "status_code": {
      "status_1xx": 0,
      "status_2xx": 0,
      "status_3xx": 0,
      "status_4xx": 1,
      "status_5xx": 0
    },
    "response_time": {
      "min": 0.0,
      "max": 0.239,
      "avg": 0.239,
      "sum": 0.239,
      "p50": 0.239,
      "p75": 0.239,
      "p90": 0.239,
      "p95": 0.239,
      "p99": 0.239
    }
  }
]
```

### Using seki with jq

The previous example can be used in conjunction with [jq](https://github.com/jqlang/jq). The following sample demonstrates sorting the `uri`s by `response_time.sum`.

```sh
$ cat access.log | seki | \
    jq 'sort_by(-.response_time.sum) | .[] | {uri: .uri, response_time_sum: .response_time.sum}'
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

If you prefer a tabular format, maybe you can use jq in conjunction with the column command to do this:

```sh
$ cat access.log | seki | \
    jq -r "sort_by(-.response_time.sum) | \
        .[] | \
        [.method + \" \" + .uri, \
        .status_code.status_2xx, \
        .status_code.status_3xx, \
        .status_code.status_4xx, \
        .status_code.status_5xx, \
        .response_time.sum] | \
        @tsv" | \
    column -t
GET  /foobar  2  0  0  0  1.208
GET  /        0  0  1  0  0.239
GET  /hello   0  1  0  0  0.113
```
