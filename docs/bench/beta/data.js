window.BENCHMARK_DATA = {
  "lastUpdate": 1630528425356,
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
      },
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
          "id": "af94982fd6a6337b40f32258fba18ea21a28bbf9",
          "message": "doc(readme): add semantic badge",
          "timestamp": "2021-09-01T22:30:29+02:00",
          "tree_id": "82c17ab0b861279ff58a60075398e937fa1aabf1",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/af94982fd6a6337b40f32258fba18ea21a28bbf9"
        },
        "date": 1630528424925,
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
            "value": 402958,
            "range": "± 100433",
            "unit": "ns/iter"
          },
          {
            "name": "Merge/merge_object",
            "value": 237,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}