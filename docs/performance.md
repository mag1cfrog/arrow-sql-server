# Performance

## Up to 4x faster than Arrow ODBC, using 92% less memory.

Arrow SQL Server has measured up to 3.9x the throughput of `arrow-odbc` in the
repository's same-data writer benchmarks. A current verification also measured
92% lower peak memory use. The comparison is specific to SQL Server bulk writes
from Arrow batches; it is not a claim about reads or general database
performance.

## Current 0.3.0 Verification

The 0.3.0 release commit was rerun on August 1, 2026 with one shared
30-million-row Arrow IPC workload:

| Backend | Rows/sec | Peak RSS | Relative throughput |
| --- | ---: | ---: | ---: |
| Arrow SQL Server `DirectRawBulk` | 682,675 | 17 MiB | 2.67x |
| `arrow-odbc` | 255,737 | 213 MiB | 1.00x |

Both backends wrote and validated the same rows. The result shows the current
release retaining a substantial throughput advantage while using roughly
one-twelfth the peak memory in this run.

Environment:

- Arrow SQL Server commit: `36610d7d1c4b399b69ea001a4abdd3a140decb86`
- scenario: `narrow_numeric`
- rows: 10,000,000 per repeat, 3 repeats
- batch size: 8,192
- SQL Server: `mcr.microsoft.com/mssql/server:2017-latest`
- `arrow-odbc`: 25.3.0
- CPU: AMD Ryzen 7 8845HS
- memory: 27 GiB
- OS: Fedora Linux 43
- Rust: 1.97.0
- Podman: 5.8.2

## Where the 4x Claim Comes From

The calibrated benchmark series used longer release-mode runs so every backend
spent about one minute or more in the measured write path. Its best
`arrow-odbc` comparison was:

| Backend | Rows/sec | Rows written | Relative throughput |
| --- | ---: | ---: | ---: |
| Arrow SQL Server `DirectRawBulk` | 1,115,473 | 75,000,000 | 3.90x |
| `arrow-odbc` | 286,151 | 75,000,000 | 1.00x |

The headline rounds that observed 3.90x result to "up to 4x." It does not claim
that every schema or SQL Server configuration is four times faster.

Other calibrated same-data comparisons from the same benchmark series found:

| Workload | Arrow SQL Server relative to `arrow-odbc` |
| --- | ---: |
| Narrow numeric columns | 3.90x |
| Mixed nullable primitives and short strings | 3.00x |
| Decimal and temporal columns | 3.31x |
| Large binary values | 1.89x |

## Why It Can Be Faster

The default `Auto` backend resolves to `DirectRawBulk`. This path:

1. binds supported Arrow arrays directly,
2. encodes SQL Server bulk rows without creating Tiberius `TokenRow` values,
3. writes bulk TDS packets through the crate's Tiberius compatibility layer.

The `arrow-odbc` comparison uses generic ODBC parameter arrays and requires an
ODBC driver. Native Microsoft ODBC BCP is a different SQL Server-specific API
and is tracked separately by the benchmark harness.

## Workload Boundaries

Performance depends on Arrow types, row width, null density, payload size,
network latency, indexes, constraints, SQL Server storage, recovery model, and
concurrent load.

In particular:

- combined large text and binary payloads have produced runs where
  `arrow-odbc` was faster,
- values that cross into SQL Server LOB storage can become dominated by SQL
  Server logging and file-flush behavior,
- short smoke runs are too sensitive to setup noise for performance claims,
- remote SQL Server instances can shift the bottleneck from encoding to
  network or server waits.

Benchmark your actual schema before using the headline as a capacity estimate.

## Reproduce the Current Comparison

Run the release-built benchmark harness:

```bash
cargo run --release -p xtask -- writer-bench compare \
  --container-runtime podman \
  --backends direct-raw,arrow-odbc \
  --scenario narrow_numeric \
  --rows 10000000 \
  --batch-size 8192 \
  --repeat 3
```

The harness generates one Arrow IPC dataset, gives it to both backends,
validates the inserted row counts, and excludes container startup, runner
compilation, dataset generation, and cleanup from write throughput.

See the [Writer Benchmark Maintainer Guide](benchmarks.md) for scenarios,
backend names, SQL Server options, metrics, and cleanup behavior.
