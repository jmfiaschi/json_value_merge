window.BENCHMARK_DATA = {
  "lastUpdate": 1630527866977,
  "repoUrl": "https://github.com/jmfiaschi/json_value_merge",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "jm.fiaschi@gmail.com",
            "name": "jmfiaschi",
            "username": "jmfiaschi"
          },
          "committer": {
            "email": "jm.fiaschi@gmail.com",
            "name": "jmfiaschi",
            "username": "jmfiaschi"
          },
          "distinct": true,
          "id": "06b8e91cd1cc81cff2c3b0a83f4b50270d844a86",
          "message": "fix(bench): rename benches for report",
          "timestamp": "2021-09-01T22:17:29+02:00",
          "tree_id": "3ef69a5f4f3fdab8d112839629bcb64643f80646",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/06b8e91cd1cc81cff2c3b0a83f4b50270d844a86"
        },
        "date": 1630527866514,
        "tool": "cargo",
        "benches": [
          {
            "name": "Merge/merge_null",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "Merge/merge_string",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "Merge/merge_array",
            "value": 436464,
            "range": "± 109078",
            "unit": "ns/iter"
          },
          {
            "name": "Merge/merge_object",
            "value": 275,
            "range": "± 6",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}