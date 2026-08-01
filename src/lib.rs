//! High-performance Apache Arrow `RecordBatch` bulk writes for Microsoft SQL Server.
//!
//! Arrow SQL Server plans Arrow schemas, generates SQL Server DDL, validates
//! target tables, and streams [`RecordBatch`] values through a SQL Server bulk
//! writer. The production path uses TDS directly and does not require an ODBC
//! driver.
//!
//! [`RecordBatch`]: arrow_array::RecordBatch
//!
//! # Start With a Schema
//!
//! Plan an Arrow schema and render matching `CREATE TABLE` SQL:
//!
//! ```
//! use arrow_schema::{DataType, Field, Schema};
//! use arrow_sql_server::{
//!     CompatibilityLevel, MssqlProfile, MssqlVersion, PlanOptions, TableName,
//!     create_table_sql_from_mappings,
//! };
//!
//! # fn main() -> arrow_sql_server::Result<()> {
//! let schema = Schema::new(vec![
//!     Field::new("id", DataType::Int64, false),
//!     Field::new("name", DataType::Utf8, true),
//! ]);
//! let profile = MssqlProfile::new(
//!     MssqlVersion::SqlServer2022,
//!     CompatibilityLevel::SQL_SERVER_2022,
//! )?;
//! let planned = profile
//!     .plan_arrow_schema(&schema, PlanOptions::default())?
//!     .into_value();
//! let table = TableName::new("dbo", "people")?;
//! let ddl = create_table_sql_from_mappings(&table, &planned);
//!
//! assert!(ddl.contains("CREATE TABLE [dbo].[people]"));
//! # Ok(())
//! # }
//! ```
//!
//! Follow the [Getting Started tutorial] for a complete connection, table
//! creation, write, and row-count verification flow.
//!
//! [Getting Started tutorial]: https://github.com/mag1cfrog/arrow-sql-server/blob/main/docs/getting-started.md
//!
//! # Core API
//!
//! - [`MssqlProfile`] and [`PlanOptions`] plan Arrow fields for a specific SQL
//!   Server version and compatibility level.
//! - [`create_table_sql_from_mappings`] renders deterministic SQL Server DDL.
//! - [`connect_mssql_client_from_ado_string`] creates a compatible asynchronous
//!   SQL Server connection.
//! - [`ConnectedMssqlClient::bulk_writer`] creates a writer for an existing
//!   target table.
//! - [`WriteOptions::default`] selects [`WriteBackend::Auto`], which currently
//!   resolves to the optimized direct raw TDS backend.
//! - [`Error::safe_error_info`] exposes sanitized, structured failure details
//!   for user-facing reports.
//!
//! # Current Scope
//!
//! This crate owns reusable Arrow-to-SQL Server planning and writing. It does
//! not provide SQL Server-to-Arrow reads, connection pooling, retries, job
//! orchestration, migrations, or multi-table publishing workflows.
//!
//! [`BulkWriter`] validates target metadata before writing. It does not create
//! or replace tables automatically.
//!
//! # Connection Compatibility
//!
//! Prefer [`connect_mssql_client_from_ado_string`] and
//! [`ConnectedMssqlClient`] in downstream applications. They hide the exact
//! `tiberius-raw-bulk` client and transport types that the writer requires.
//!
//! # Guides and Reference
//!
//! - [Type Mapping](https://github.com/mag1cfrog/arrow-sql-server/blob/main/docs/type-mapping.md)
//! - [Performance](https://github.com/mag1cfrog/arrow-sql-server/blob/main/docs/performance.md)
//! - [Observability](https://github.com/mag1cfrog/arrow-sql-server/blob/main/docs/observability.md)
//! - [Documentation Index](https://github.com/mag1cfrog/arrow-sql-server/blob/main/docs/README.md)

/// Arrow-side schema metadata.
pub mod arrow;
/// SQL Server connection helpers.
pub mod connection;
/// Directional conversion semantics between Arrow and SQL Server.
pub(crate) mod conversion;
/// Structured diagnostics for planning and writing.
pub mod diagnostic;
/// Error types for Arrow SQL Server.
pub mod error;
/// MSSQL-side schema metadata, identifiers, profiles, types, and DDL helpers.
pub mod mssql;
mod observability;
/// Bidirectional Arrow/MSSQL schema mapping.
pub mod schema;
/// Write-path options and conversion policies.
pub mod write;

pub use arrow::ArrowFieldRef;
pub use connection::{
    ConnectedBulkWriter, ConnectedMssqlClient, SqlExecutionOutcome,
    connect_mssql_client_from_ado_string,
};
pub use diagnostic::{
    Diagnostic, DiagnosticCode, DiagnosticSet, DiagnosticSeverity, FieldRef, PlanOutcome,
};
pub use error::{Error, ErrorInfo, Result};
pub use mssql::{
    CompatibilityLevel, CreateTableOptions, Identifier, IdentifierPolicy, MssqlColumn,
    MssqlProfile, MssqlTimePrecision, MssqlType, MssqlTypeLength, MssqlVersion, TableName,
    create_table_sql,
};
#[cfg(test)]
pub(crate) use schema::plan_arrow_schema_to_mssql_mappings;
pub use schema::{
    PlannedSchema, SchemaMapping, create_table_sql_from_mappings, mssql_columns_from_mappings,
    plan_arrow_schema_to_mssql_schema,
};
pub use write::{
    BinaryPolicy, BulkWriter, Date64Policy, Decimal256Policy, DecimalPolicy, FloatPolicy,
    NanosecondPolicy, PlanOptions, SchemaCheck, StringPolicy, TimestampPolicy, TimezonePolicy,
    UInt64Policy, WriteBackend, WriteOptions, WritePhase, WriteStats,
    validate_arrow_schema_against_mappings, validate_record_batch_schema_against_mappings,
};
