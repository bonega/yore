window.BENCHMARK_DATA = {
  "lastUpdate": 1768857704933,
  "repoUrl": "https://github.com/bonega/yore",
  "entries": {
    "Yore Benchmarks": [
      {
        "commit": {
          "author": {
            "email": "bonega@gmail.com",
            "name": "Andreas Liljeqvist",
            "username": "bonega"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dd079162000c7b7f7828f1f87305c58dcaa58ecd",
          "message": "ci: add benchmark job with gh-pages storage (#21)\n\nAdd benchmark job to CI workflow using benchmark-action/github-action-benchmark\nto track performance over time. Results are stored in gh-pages branch.\n\n- Runs benchmarks with bencher output format\n- Alerts on >10% regression, fails on >20%\n- Auto-pushes results to gh-pages on master\n- Comments benchmark comparisons on PRs",
          "timestamp": "2026-01-19T22:19:34+01:00",
          "tree_id": "b421101c72fc261fcb04d25ac66ae32bd5700336",
          "url": "https://github.com/bonega/yore/commit/dd079162000c7b7f7828f1f87305c58dcaa58ecd"
        },
        "date": 1768857704701,
        "tool": "cargo",
        "benches": [
          {
            "name": "decode_checked/mostly_ascii/8",
            "value": 34,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/64",
            "value": 70,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/256",
            "value": 143,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/512",
            "value": 228,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/1024",
            "value": 423,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/2048",
            "value": 888,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/4096",
            "value": 2004,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/8",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/64",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/256",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/512",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/1024",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/2048",
            "value": 50,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/4096",
            "value": 91,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/8",
            "value": 36,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/64",
            "value": 85,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/256",
            "value": 260,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/512",
            "value": 485,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/1024",
            "value": 902,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/2048",
            "value": 1760,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/4096",
            "value": 3478,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/8",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/64",
            "value": 63,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/256",
            "value": 173,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/512",
            "value": 323,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/1024",
            "value": 622,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/2048",
            "value": 1220,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/4096",
            "value": 2394,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/8",
            "value": 39,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/64",
            "value": 65,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/256",
            "value": 148,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/512",
            "value": 236,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/1024",
            "value": 414,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/2048",
            "value": 789,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/4096",
            "value": 1545,
            "range": "± 55",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/8",
            "value": 42,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/64",
            "value": 128,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/256",
            "value": 457,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/512",
            "value": 911,
            "range": "± 180",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/1024",
            "value": 1801,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/2048",
            "value": 3541,
            "range": "± 331",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/4096",
            "value": 7027,
            "range": "± 206",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/ascii/8",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/ascii/64",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/ascii/256",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/ascii/512",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/ascii/1024",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/ascii/2048",
            "value": 50,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/ascii/4096",
            "value": 89,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/8",
            "value": 52,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/64",
            "value": 208,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/256",
            "value": 705,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/512",
            "value": 1382,
            "range": "± 283",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/1024",
            "value": 2747,
            "range": "± 162",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/2048",
            "value": 5425,
            "range": "± 338",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/4096",
            "value": 10785,
            "range": "± 218",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/8",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/64",
            "value": 227,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/256",
            "value": 797,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/512",
            "value": 1559,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/1024",
            "value": 3094,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/2048",
            "value": 6138,
            "range": "± 129",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/4096",
            "value": 12228,
            "range": "± 90",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}