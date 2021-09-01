window.BENCHMARK_DATA = {
  "lastUpdate": 1630528502812,
  "repoUrl": "https://github.com/jmfiaschi/json_value_merge",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "jm.fiaschi@gmail.com",
            "name": "jm.fiaschi",
            "username": "jmfiaschi"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "febb0d0c09aac63a87e89e73aa6ef1e5947eec41",
          "message": "feat(ci): lint & ci",
          "timestamp": "2021-09-01T22:27:03+02:00",
          "tree_id": "3ef69a5f4f3fdab8d112839629bcb64643f80646",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/febb0d0c09aac63a87e89e73aa6ef1e5947eec41"
        },
        "date": 1630528500797,
        "tool": "cargo",
        "benches": [
          {
            "name": "Merge/merge_null",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "Merge/merge_string",
            "value": 39,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "Merge/merge_array",
            "value": 425570,
            "range": "± 96881",
            "unit": "ns/iter"
          },
          {
            "name": "Merge/merge_object",
            "value": 245,
            "range": "± 11",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}