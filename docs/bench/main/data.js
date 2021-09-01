window.BENCHMARK_DATA = {
  "lastUpdate": 1630529760000,
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
          "id": "eff396effbde3fe91344114809641b377fd35f7c",
          "message": "feat(project): force semantic release",
          "timestamp": "2021-09-01T22:51:24+02:00",
          "tree_id": "18aaaaa1ba5b1834bf3033beea39589bebba73a8",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/eff396effbde3fe91344114809641b377fd35f7c"
        },
        "date": 1630529759543,
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
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "Merge/merge_array",
            "value": 393342,
            "range": "± 100160",
            "unit": "ns/iter"
          },
          {
            "name": "Merge/merge_object",
            "value": 238,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}