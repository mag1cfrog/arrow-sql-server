<h1 align="center">Arrow SQL Server</h1>

<p align="center">
  <a href="https://crates.io/crates/arrow-sql-server"><img alt="Crates.io" src="https://img.shields.io/crates/v/arrow-sql-server.svg"></a>
  <a href="https://docs.rs/arrow-sql-server"><img alt="Docs.rs" src="https://docs.rs/arrow-sql-server/badge.svg"></a>
  <a href="LICENSE"><img alt="Apache 2.0 license" src="https://img.shields.io/badge/license-Apache--2.0-blue.svg"></a>
</p>

<p align="center">
  <strong>Bulk-write Apache Arrow RecordBatch values to Microsoft SQL Server without an ODBC driver.</strong>
</p>

<h2 align="center">
  Up to 4x faster than Arrow ODBC.<br>
  Uses 92% less memory.
</h2>

<p align="center">
  Measured up to 3.9x the throughput of Arrow ODBC.<br>
  A fresh 0.3.0 run measured 2.67x the throughput and 17 MiB versus 213 MiB peak memory.<br>
  <a href="docs/performance.md">See the results, limitations, and reproduction command.</a>
</p>

Arrow SQL Server is a Rust library for schema-aware, asynchronous SQL Server
bulk loading. It plans Arrow schemas, generates SQL Server DDL, validates target
tables, and writes batches through a direct Arrow-to-TDS path.

## Why Arrow SQL Server?

- **Built for Arrow:** write `RecordBatch` values directly instead of converting
  them into application row objects.
- **Built for SQL Server:** explicit type planning, compatibility profiles,
  quoted identifiers, target-table validation, and bulk-load diagnostics.
- **No ODBC runtime:** the production path uses TDS through Tiberius, so your
  application does not need unixODBC or a Microsoft ODBC driver.

## Is It a Good Fit?

| Use Arrow SQL Server when you need | This crate does not provide |
| --- | --- |
| Arrow-to-SQL Server bulk writes from Rust | SQL Server-to-Arrow reads |
| Streaming writes across one or more batches | Connection pooling, retries, or job orchestration |
| SQL Server-aware schema planning and DDL | A database-agnostic writer abstraction |
| A direct TDS path without an ODBC deployment | Migrations, an ORM, or automatic table publishing workflows |

The crate can generate `CREATE TABLE` SQL, but it does not create or replace a
table unless your application explicitly executes that SQL.

## Install

Arrow SQL Server 0.3 uses Arrow 58 types. Add the crates used by the examples:

```bash
cargo add arrow-sql-server@0.3 arrow-array@58 arrow-schema@58
cargo add tokio@1 --features macros,rt
```

The minimum supported Rust version is 1.88.

## Write a Batch

The target table must already match the planned schema. New applications should
use `WriteOptions::default()`; its `Auto` backend selects the optimized writer.

```rust
use arrow_array::RecordBatch;
use arrow_sql_server::{
    CompatibilityLevel, MssqlProfile, MssqlVersion, PlanOptions, TableName,
    WriteOptions, WriteStats, connect_mssql_client_from_ado_string,
};

async fn write_batch(
    connection_string: &str,
    batch: &RecordBatch,
) -> arrow_sql_server::Result<WriteStats> {
    let profile = MssqlProfile::new(
        MssqlVersion::SqlServer2022,
        CompatibilityLevel::SQL_SERVER_2022,
    )?;
    let planned_schema = profile
        .plan_arrow_schema(batch.schema().as_ref(), PlanOptions::default())?
        .into_value();

    let table = TableName::new("dbo", "people")?;
    let mut client = connect_mssql_client_from_ado_string(connection_string).await?;
    let mut writer = client
        .bulk_writer(table, planned_schema, WriteOptions::default())
        .await?;

    writer.write_batch(batch).await?;
    writer.finish().await
}
```

For a complete first run that creates a table, writes a batch, and verifies the
row count, follow [Getting Started](docs/getting-started.md).

## Supported Data

The default planner and both production writers support common Arrow scalar
types, including:

- booleans and signed or unsigned integers,
- floating-point values,
- UTF-8 and binary arrays, including Arrow view arrays,
- decimal values up to SQL Server precision 38,
- dates, times, timestamps, and timezone-aware timestamps.

Nested Arrow values and SQL Server-to-Arrow reads are not currently supported.
See the [complete type-mapping reference](docs/type-mapping.md) for policies,
runtime checks, and unsupported types.

SQL Server profiles cover SQL Server 2016, 2017, 2019, 2022, and 2025 with
compatibility-level validation.

## Learn More

Start here:

- [Getting Started](docs/getting-started.md): complete your first SQL Server
  write.
- [Type Mapping Reference](docs/type-mapping.md): check supported Arrow and SQL
  Server types.
- [Performance](docs/performance.md): understand the benchmark claim and its
  workload boundaries.
- [API Documentation](https://docs.rs/arrow-sql-server): browse public Rust
  types and methods.

Advanced and maintainer documentation:

- [Observability](docs/observability.md)
- [Documentation Index](docs/README.md)
