window.BENCHMARK_DATA = {
  "lastUpdate": 1736626736246,
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
          "id": "7292cb685b3e8e8484ec2bed3f4e0ef1cc470009",
          "message": "feat(merge_in): fix bench & clean the code & add complex merge in bench (#11)\n\n* feat(merge_in): fix bench & clean the code & add complex merge in bench\r\n\r\n* feat(merge_in): add wildcard for array\r\n\r\n* refacto(bench): reword labels\r\n\r\n* fix(merge_in): char not covered and create an infinit loop\r\n\r\n* feat(mergin_in): raise exception if try merge object in array with wrong json pointer",
          "timestamp": "2021-09-17T13:58:31+02:00",
          "tree_id": "6e29698545c38954db777c0d898a6976ef05a603",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/7292cb685b3e8e8484ec2bed3f4e0ef1cc470009"
        },
        "date": 1631880419811,
        "tool": "cargo",
        "benches": [
          {
            "name": "merge/null",
            "value": 32,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "merge/string",
            "value": 66,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "merge/array",
            "value": 321,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "merge/object",
            "value": 276,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/null",
            "value": 441,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/object",
            "value": 635,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/array",
            "value": 455,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/deeper_object",
            "value": 15067,
            "range": "± 835",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "blitzkrieg263@gmail.com",
            "name": "Jean-Baptiste Trystram",
            "username": "jbtrystram"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3b581a4c8e6b1da0c7ecfc1d5078cf634a103f95",
          "message": "Fix a typo in liscense filenames\n\nThe license files are names `LICENSE-*` but were imported as `LICENCE-*`.",
          "timestamp": "2021-12-06T22:57:35+01:00",
          "tree_id": "add3afefa086e3a20f7a805a931cce097bb1f09b",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/3b581a4c8e6b1da0c7ecfc1d5078cf634a103f95"
        },
        "date": 1638828313953,
        "tool": "cargo",
        "benches": [
          {
            "name": "merge/null",
            "value": 25,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "merge/string",
            "value": 54,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "merge/array",
            "value": 272,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "merge/object",
            "value": 248,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/null",
            "value": 409,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/string.",
            "value": 492,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/object",
            "value": 595,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/array",
            "value": 416,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/deeper_object",
            "value": 14007,
            "range": "± 745",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "59c90bd90099e3946a695e10ed77825b4e7f6fa5",
          "message": "fix(version): update version and publish licenses",
          "timestamp": "2021-12-06T23:19:30+01:00",
          "tree_id": "9d76ca15916849fa8df1da61cf10557b31a3c78e",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/59c90bd90099e3946a695e10ed77825b4e7f6fa5"
        },
        "date": 1638829406382,
        "tool": "cargo",
        "benches": [
          {
            "name": "merge/null",
            "value": 25,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "merge/string",
            "value": 50,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "merge/array",
            "value": 291,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "merge/object",
            "value": 220,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/null",
            "value": 379,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/string.",
            "value": 474,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/object",
            "value": 561,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/array",
            "value": 410,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/deeper_object",
            "value": 14721,
            "range": "± 1076",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "785df9e77659a0a2a35a5a0df3ec6e1e5675d8f0",
          "message": "fix(cargo): update version automatically",
          "timestamp": "2021-12-17T23:19:08+01:00",
          "tree_id": "99c58d05a758c0b2666eeb6651b72d49adcec0b7",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/785df9e77659a0a2a35a5a0df3ec6e1e5675d8f0"
        },
        "date": 1639779969224,
        "tool": "cargo",
        "benches": [
          {
            "name": "merge/null",
            "value": 27,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "merge/string",
            "value": 58,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "merge/array",
            "value": 271,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "merge/object",
            "value": 229,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/null",
            "value": 393,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/string.",
            "value": 482,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/object",
            "value": 581,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/array",
            "value": 422,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/deeper_object",
            "value": 13724,
            "range": "± 462",
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
          "id": "af54495effc020548a3d0156d8c00c3263e93c07",
          "message": "chore(release): commit version",
          "timestamp": "2021-12-19T11:28:29+01:00",
          "tree_id": "657fa28e71b8b02f59426f6280ef890ae8f7ef48",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/af54495effc020548a3d0156d8c00c3263e93c07"
        },
        "date": 1639909954368,
        "tool": "cargo",
        "benches": [
          {
            "name": "merge/null",
            "value": 25,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "merge/string",
            "value": 54,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "merge/array",
            "value": 264,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "merge/object",
            "value": 220,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/null",
            "value": 365,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/string.",
            "value": 442,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/object",
            "value": 562,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/array",
            "value": 388,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/deeper_object",
            "value": 13618,
            "range": "± 1077",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "344fd31d483004b2821a110345052b600c4c6752",
          "message": "refactor(merge_in): Improve performance (#15)\n\n* refactor(merge_in): Improve performance\r\n* refactor(cicd): fix semantic release",
          "timestamp": "2023-09-09T21:39:42+02:00",
          "tree_id": "0eb97a84833c49179dfe16522fcd8c30677e88b2",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/344fd31d483004b2821a110345052b600c4c6752"
        },
        "date": 1694288793672,
        "tool": "cargo",
        "benches": [
          {
            "name": "merge/null",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge/string",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge/array",
            "value": 148,
            "range": "± 394",
            "unit": "ns/iter"
          },
          {
            "name": "merge/object",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/null",
            "value": 257,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/string.",
            "value": 275,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/object",
            "value": 319,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/array",
            "value": 288,
            "range": "± 516",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/deeper_object",
            "value": 1674,
            "range": "± 55",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "ff3843743b44b842e56263a9c297e90577b3f9c1",
          "message": "feat(perf): force new version (#17)\n\n* feat(perf): force new version\r\n* feat(makefile): add version command",
          "timestamp": "2023-09-09T23:12:32+02:00",
          "tree_id": "d9a9cb5b18f366e2bcad55df385946d9863234aa",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/ff3843743b44b842e56263a9c297e90577b3f9c1"
        },
        "date": 1694294266098,
        "tool": "cargo",
        "benches": [
          {
            "name": "merge/null",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge/string",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge/array",
            "value": 128,
            "range": "± 388",
            "unit": "ns/iter"
          },
          {
            "name": "merge/object",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/null",
            "value": 233,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/string.",
            "value": 248,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/object",
            "value": 293,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/array",
            "value": 266,
            "range": "± 99",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/deeper_object",
            "value": 1573,
            "range": "± 28",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "052e37f6f70698d7782caf86b577cb836e18d578",
          "message": "perf(merge): apply pointer in arguments and allow to replace array by object\n\nBREAKING CHANGE: merge with pointer and all replace array by object",
          "timestamp": "2023-09-14T22:57:25+02:00",
          "tree_id": "ba5cd384d28a36ee71d2d5bff714eadaa4a17450",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/052e37f6f70698d7782caf86b577cb836e18d578"
        },
        "date": 1694725432088,
        "tool": "cargo",
        "benches": [
          {
            "name": "merge/null",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge/string",
            "value": 24,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "merge/array",
            "value": 159,
            "range": "± 396",
            "unit": "ns/iter"
          },
          {
            "name": "merge/object",
            "value": 66,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/null",
            "value": 314,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/string.",
            "value": 331,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/object",
            "value": 387,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/array",
            "value": 350,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/deeper_object",
            "value": 1995,
            "range": "± 70",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "916a8486a05aa3d1ab330b0d952c76e2da24c5c8",
          "message": "Merge pull request #20 from jmfiaschi/beta\n\nrefactor(merge_in): Improve perf",
          "timestamp": "2025-01-09T22:16:42+01:00",
          "tree_id": "39e75f10ac557afac6826f3a868e1f192f9b4317",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/916a8486a05aa3d1ab330b0d952c76e2da24c5c8"
        },
        "date": 1736458133815,
        "tool": "cargo",
        "benches": [
          {
            "name": "merge/null",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge/string",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge/array",
            "value": 67,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "merge/object",
            "value": 38,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/null",
            "value": 185,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/string.",
            "value": 188,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/object",
            "value": 211,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/array",
            "value": 175,
            "range": "± 3485",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/deeper_object",
            "value": 1141,
            "range": "± 310",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jm.fiaschi@gmail.com",
            "name": "jm.fiaschi",
            "username": "jmfiaschi"
          },
          "committer": {
            "email": "jm.fiaschi@gmail.com",
            "name": "Jean-Marc FIASCHI",
            "username": "jmfiaschi"
          },
          "distinct": true,
          "id": "023363e146b2f0b8991df36cd323724408f5a10c",
          "message": "refactor(merge_in): Improve perf",
          "timestamp": "2025-01-09T22:38:49+01:00",
          "tree_id": "39e75f10ac557afac6826f3a868e1f192f9b4317",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/023363e146b2f0b8991df36cd323724408f5a10c"
        },
        "date": 1736459003831,
        "tool": "cargo",
        "benches": [
          {
            "name": "merge/null",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge/string",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge/array",
            "value": 69,
            "range": "± 69",
            "unit": "ns/iter"
          },
          {
            "name": "merge/object",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/null",
            "value": 178,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/string.",
            "value": 193,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/object",
            "value": 209,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/array",
            "value": 165,
            "range": "± 474",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/deeper_object",
            "value": 1149,
            "range": "± 82",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jm.fiaschi@gmail.com",
            "name": "jm.fiaschi",
            "username": "jmfiaschi"
          },
          "committer": {
            "email": "jm.fiaschi@gmail.com",
            "name": "Jean-Marc FIASCHI",
            "username": "jmfiaschi"
          },
          "distinct": true,
          "id": "ab43db714d79ef04fd091c3228b8a78ae5d54c8d",
          "message": "fix(merge): Improve perf",
          "timestamp": "2025-01-10T09:27:14+01:00",
          "tree_id": "39e75f10ac557afac6826f3a868e1f192f9b4317",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/ab43db714d79ef04fd091c3228b8a78ae5d54c8d"
        },
        "date": 1736497951659,
        "tool": "cargo",
        "benches": [
          {
            "name": "merge/null",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge/string",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge/array",
            "value": 68,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "merge/object",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/null",
            "value": 168,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/string.",
            "value": 179,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/object",
            "value": 201,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/array",
            "value": 171,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/deeper_object",
            "value": 1112,
            "range": "± 233",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jm.fiaschi@gmail.com",
            "name": "Jean-Marc FIASCHI",
            "username": "jmfiaschi"
          },
          "committer": {
            "email": "jm.fiaschi@gmail.com",
            "name": "Jean-Marc FIASCHI",
            "username": "jmfiaschi"
          },
          "distinct": true,
          "id": "c2513a8a20c9809bde404a1a5747726f863121fe",
          "message": "chore(readme): update description",
          "timestamp": "2025-01-11T21:12:15+01:00",
          "tree_id": "d9e0026a042d924a19ae7545a7791099046b2059",
          "url": "https://github.com/jmfiaschi/json_value_merge/commit/c2513a8a20c9809bde404a1a5747726f863121fe"
        },
        "date": 1736626735661,
        "tool": "cargo",
        "benches": [
          {
            "name": "merge/null",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge/string",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "merge/array",
            "value": 69,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "merge/object",
            "value": 39,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/null",
            "value": 182,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/string.",
            "value": 197,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/object",
            "value": 216,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/array",
            "value": 168,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "merge_in/deeper_object",
            "value": 1099,
            "range": "± 98",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}