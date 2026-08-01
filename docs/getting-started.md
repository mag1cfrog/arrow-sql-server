# Getting Started

This tutorial creates one Arrow batch, creates a matching SQL Server table,
writes the batch, and verifies the stored row count.

## Prerequisites

You need:

- Rust 1.88 or newer,
- a reachable SQL Server instance,
- a database user allowed to create a table and insert rows,
- an ADO-style SQL Server connection string.

The tutorial uses a table named `[dbo].[arrow_sql_server_quickstart]`. It stops
without changing anything if that table already exists.

## Create the Project

```bash
cargo new arrow-sql-server-quickstart
cd arrow-sql-server-quickstart
cargo add arrow-sql-server@0.3 arrow-array@58 arrow-schema@58
cargo add tokio@1 --features macros,rt
```

## Write the Program

Replace `src/main.rs` with:

```rust
use std::{env, error::Error, io, sync::Arc};

use arrow_array::{ArrayRef, Int64Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use arrow_sql_server::{
    CompatibilityLevel, MssqlProfile, MssqlVersion, PlanOptions, TableName,
    WriteOptions, connect_mssql_client_from_ado_string,
    create_table_sql_from_mappings,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection_string = env::var("MSSQL_URL")?;
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("name", DataType::Utf8, true),
    ]));
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(Int64Array::from(vec![1, 2, 3])) as ArrayRef,
            Arc::new(StringArray::from(vec![
                Some("Ada"),
                Some("Grace"),
                None,
            ])) as ArrayRef,
        ],
    )?;

    let profile = MssqlProfile::new(
        MssqlVersion::SqlServer2022,
        CompatibilityLevel::SQL_SERVER_2022,
    )?;
    let planned_schema = profile
        .plan_arrow_schema(schema.as_ref(), PlanOptions::default())?
        .into_value();
    let table = TableName::new("dbo", "arrow_sql_server_quickstart")?;
    let create_table_sql = create_table_sql_from_mappings(&table, &planned_schema);

    let mut client = connect_mssql_client_from_ado_string(&connection_string).await?;
    if client.table_exists(&table).await? {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("{} already exists", table.quoted_sql()),
        )
        .into());
    }
    client.execute_statement(&create_table_sql).await?;

    let mut writer = client
        .bulk_writer(
            table.clone(),
            planned_schema,
            WriteOptions::default(),
        )
        .await?;
    writer.write_batch(&batch).await?;
    let stats = writer.finish().await?;

    let stored_rows = client.target_row_count(&table).await?;
    assert_eq!(stats.rows_written, 3);
    assert_eq!(stored_rows, 3);
    println!("wrote {stored_rows} rows to {}", table.quoted_sql());

    Ok(())
}
```

Choose the `MssqlVersion` and `CompatibilityLevel` values that match your
database. The example uses SQL Server 2022 at compatibility level 160.

## Run It

Set the connection string and run the program:

```bash
MSSQL_URL='server=tcp:127.0.0.1,1433;user id=sa;password=...;TrustServerCertificate=true' \
  cargo run
```

Expected output:

```text
wrote 3 rows to [dbo].[arrow_sql_server_quickstart]
```

The table is intentionally left in place so you can inspect it. Remove it when
you no longer need it.

## Use an Existing Table

For an existing table, skip `create_table_sql_from_mappings` and
`execute_statement`. The writer validates SQL Server metadata before accepting
rows. A column name, order, SQL type, or nullability mismatch returns an error
before the bulk write starts.

## Next Steps

- Check [Type Mapping](type-mapping.md) before introducing additional Arrow
  types or non-default conversion policies.
- Read [Performance](performance.md) for benchmark results and workload
  boundaries.
- Add [Observability](observability.md) when the write is part of a service or
  scheduled workflow.
- Run the repository's
  [`sqlserver_batch_write`](../examples/sqlserver_batch_write.rs) example for a
  multi-batch variant with optional cleanup controls.
