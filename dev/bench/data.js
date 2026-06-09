window.BENCHMARK_DATA = {
  "lastUpdate": 1781034543735,
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
      },
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
          "id": "65ad00d8c0db0efac173a09f885508d9e1ac2037",
          "message": "perf: add AVX512-optimized is_ascii to fix native CPU regression (#22)\n\nWhen compiling with `-C target-cpu=native` on AVX512 CPUs, LLVM generates\npoor code for stdlib's is_ascii(), causing 30x slowdowns. This adds a custom\nimplementation using AVX512BW intrinsics with runtime feature detection.\n\n- Add src/simd.rs with is_ascii() using _mm512_movepi8_mask\n- Use masked loads for safe tail handling (<64 bytes)\n- Runtime detection via is_x86_feature_detected!(\"avx512bw\")\n- Falls back to stdlib is_ascii() on non-AVX512 systems\n\nBenchmarks vs master:\n- ASCII workloads: 1.3-1.8x faster (default compilation)\n- ASCII workloads: 30-35x faster (with -C target-cpu=native)",
          "timestamp": "2026-01-22T22:12:53Z",
          "tree_id": "bd825e6dda8d70762783baa4fb1ac4417850f2a7",
          "url": "https://github.com/bonega/yore/commit/65ad00d8c0db0efac173a09f885508d9e1ac2037"
        },
        "date": 1769120103156,
        "tool": "cargo",
        "benches": [
          {
            "name": "decode_checked/mostly_ascii/8",
            "value": 34,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/64",
            "value": 67,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/256",
            "value": 147,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/512",
            "value": 240,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/1024",
            "value": 419,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/2048",
            "value": 911,
            "range": "± 123",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/4096",
            "value": 1998,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/8",
            "value": 9,
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
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/1024",
            "value": 26,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/2048",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/4096",
            "value": 92,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/8",
            "value": 37,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/64",
            "value": 87,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/256",
            "value": 266,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/512",
            "value": 493,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/1024",
            "value": 910,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/2048",
            "value": 1765,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/4096",
            "value": 3481,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/8",
            "value": 48,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/64",
            "value": 64,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/256",
            "value": 178,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/512",
            "value": 333,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/1024",
            "value": 646,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/2048",
            "value": 1272,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/4096",
            "value": 2491,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/8",
            "value": 48,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/64",
            "value": 66,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/256",
            "value": 154,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/512",
            "value": 246,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/1024",
            "value": 431,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/2048",
            "value": 810,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/4096",
            "value": 1571,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/8",
            "value": 43,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/64",
            "value": 139,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/256",
            "value": 481,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/512",
            "value": 939,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/1024",
            "value": 2005,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/2048",
            "value": 4090,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/4096",
            "value": 7311,
            "range": "± 309",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/ascii/8",
            "value": 9,
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
            "value": 10,
            "range": "± 1",
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
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/ascii/4096",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/8",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/64",
            "value": 202,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/256",
            "value": 705,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/512",
            "value": 1375,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/1024",
            "value": 2726,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/2048",
            "value": 5400,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/4096",
            "value": 10749,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/8",
            "value": 54,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/64",
            "value": 247,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/256",
            "value": 877,
            "range": "± 141",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/512",
            "value": 1717,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/1024",
            "value": 3412,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/2048",
            "value": 6764,
            "range": "± 131",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/4096",
            "value": 13475,
            "range": "± 61",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "c6a2ee5b0ec196716e317a217695a73676efaf4a",
          "message": "Revert \"perf: add AVX512-optimized is_ascii to fix native CPU regression (#22)\" (#23)\n\nThis reverts commit 65ad00d8c0db0efac173a09f885508d9e1ac2037.",
          "timestamp": "2026-06-09T19:34:13Z",
          "tree_id": "9345bfd822c9229dd2e95cd734e150c52fba5203",
          "url": "https://github.com/bonega/yore/commit/c6a2ee5b0ec196716e317a217695a73676efaf4a"
        },
        "date": 1781033804646,
        "tool": "cargo",
        "benches": [
          {
            "name": "decode_checked/mostly_ascii/8",
            "value": 32,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/64",
            "value": 63,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/256",
            "value": 128,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/512",
            "value": 207,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/1024",
            "value": 369,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/2048",
            "value": 790,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/4096",
            "value": 1785,
            "range": "± 55",
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
            "range": "± 1",
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
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/2048",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/4096",
            "value": 89,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/8",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/64",
            "value": 84,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/256",
            "value": 259,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/512",
            "value": 476,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/1024",
            "value": 901,
            "range": "± 149",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/2048",
            "value": 1750,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/4096",
            "value": 3455,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/8",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/64",
            "value": 63,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/256",
            "value": 174,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/512",
            "value": 323,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/1024",
            "value": 625,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/2048",
            "value": 1211,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/4096",
            "value": 2403,
            "range": "± 42",
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
            "value": 72,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/256",
            "value": 156,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/512",
            "value": 245,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/1024",
            "value": 428,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/2048",
            "value": 798,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/4096",
            "value": 1551,
            "range": "± 49",
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
            "value": 139,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/256",
            "value": 491,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/512",
            "value": 924,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/1024",
            "value": 1815,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/2048",
            "value": 4080,
            "range": "± 157",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/4096",
            "value": 8895,
            "range": "± 202",
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
            "value": 49,
            "range": "± 0",
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/64",
            "value": 202,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/256",
            "value": 706,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/512",
            "value": 1379,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/1024",
            "value": 2740,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/2048",
            "value": 5407,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/4096",
            "value": 10766,
            "range": "± 607",
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
            "value": 229,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/256",
            "value": 801,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/512",
            "value": 1565,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/1024",
            "value": 3101,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/2048",
            "value": 6135,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/4096",
            "value": 12220,
            "range": "± 173",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "8c9d1d3241db9c3644798c42cf02820085885cac",
          "message": "Remove thiserror dependency in favor of hand-written error impls (#24)\n\nyore only used thiserror for Display/Error on two trivial error types\n(EncodeError, DecodeError) with no #[from]/#[source]/source chaining.\nReplacing the derive with hand-written impls makes yore dependency-free.\n\nThe public API is unchanged (struct names, fields, and Debug/Display/Error\nimpls are identical, error messages byte-for-byte), so this is a\nnon-breaking patch release: 1.3.0 -> 1.3.1.",
          "timestamp": "2026-06-09T19:46:29Z",
          "tree_id": "3ff91396167a50a7c66c50eabeacbbb39db60c00",
          "url": "https://github.com/bonega/yore/commit/8c9d1d3241db9c3644798c42cf02820085885cac"
        },
        "date": 1781034542927,
        "tool": "cargo",
        "benches": [
          {
            "name": "decode_checked/mostly_ascii/8",
            "value": 32,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/64",
            "value": 73,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/256",
            "value": 144,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/512",
            "value": 230,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/1024",
            "value": 410,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/2048",
            "value": 863,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/mostly_ascii/4096",
            "value": 1925,
            "range": "± 79",
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/ascii/4096",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/8",
            "value": 35,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/64",
            "value": 85,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/256",
            "value": 272,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/512",
            "value": 486,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/1024",
            "value": 905,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/2048",
            "value": 1758,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "decode_checked/extended/4096",
            "value": 3462,
            "range": "± 34",
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
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/256",
            "value": 177,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/512",
            "value": 333,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/1024",
            "value": 645,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/2048",
            "value": 1258,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/all_bad/4096",
            "value": 2486,
            "range": "± 31",
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
            "value": 64,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/256",
            "value": 150,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/512",
            "value": 243,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/1024",
            "value": 428,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/2048",
            "value": 808,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "decode_lossy/mostly_ascii/4096",
            "value": 1568,
            "range": "± 49",
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
            "value": 140,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/256",
            "value": 482,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/512",
            "value": 968,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/1024",
            "value": 1940,
            "range": "± 113",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/2048",
            "value": 4227,
            "range": "± 164",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/mostly_ascii/4096",
            "value": 8911,
            "range": "± 200",
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
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/ascii/4096",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/8",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/64",
            "value": 203,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/256",
            "value": 707,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/512",
            "value": 1378,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/1024",
            "value": 2732,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/2048",
            "value": 5408,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "encode_checked/extended/4096",
            "value": 10777,
            "range": "± 48",
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
            "value": 799,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/512",
            "value": 1562,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/1024",
            "value": 3095,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/2048",
            "value": 6135,
            "range": "± 1177",
            "unit": "ns/iter"
          },
          {
            "name": "encode_lossy/all_bad/4096",
            "value": 12224,
            "range": "± 165",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}