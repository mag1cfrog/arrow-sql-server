# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/mag1cfrog/arrow-sql-server/compare/v0.3.2...v0.4.0) - 2026-08-05

### Added

- add opt-in ASCII varchar string policy ([#201](https://github.com/mag1cfrog/arrow-sql-server/pull/201))

## [0.3.2](https://github.com/mag1cfrog/arrow-sql-server/compare/v0.3.1...v0.3.2) - 2026-08-04

### Added

- support optional table locking for SQL Server bulk loads

### Other

- optimize ASCII nvarchar encoding in the direct writer

## [0.3.1](https://github.com/mag1cfrog/arrow-sql-server/compare/v0.3.0...v0.3.1) - 2026-08-01

### Other

- improve newcomer onboarding and performance story ([#196](https://github.com/mag1cfrog/arrow-sql-server/pull/196))

## [0.3.0](https://github.com/mag1cfrog/arrow-sql-server/compare/v0.2.1...v0.3.0) - 2026-08-01

### Changed

- rename the package from `arrow-tiberius` to `arrow-sql-server` and the Rust import from `arrow_tiberius` to `arrow_sql_server`
- rename project-owned tracing, environment, SQL object, Docker, test, and benchmark namespaces to Arrow SQL Server forms
- require consumers to update Cargo dependencies and Rust imports directly; no compatibility package, alias, or re-export is provided
- keep existing `arrow-tiberius` releases available on crates.io as historical versions while new releases continue as `arrow-sql-server`

## [0.2.1](https://github.com/mag1cfrog/arrow-sql-server/compare/v0.2.0...v0.2.1) - 2026-07-21

### Added

- expose detailed SQL Server bulk finalization tracing ([#183](https://github.com/mag1cfrog/arrow-sql-server/pull/183))

### Other

- refresh public docs for 0.2 profiles ([#181](https://github.com/mag1cfrog/arrow-sql-server/pull/181))

## [0.2.0](https://github.com/mag1cfrog/arrow-sql-server/compare/v0.1.6...v0.2.0) - 2026-07-07

### Added

- support SQL Server 2019, 2022, and 2025 profiles with compatibility-level validation
- *(write)* add SQL Server compatibility profiles and profile-bound write planning
- *(write)* expose safe phase, cause, and diagnostic details for write failures

### Fixed

- preserve SQL Server datetime compatibility-level rounding for timestamp writes
- *(ci)* run release-plz with a Rust toolchain compatible with semver checks
- *(write)* allow DirectRawBulk to write non-null Arrow timestamps as SQL Server datetime

## [0.1.6](https://github.com/mag1cfrog/arrow-sql-server/compare/v0.1.5...v0.1.6) - 2026-07-04

### Added

- *(write)* support target-aware timestamp writes for datetime types

## [0.1.5](https://github.com/mag1cfrog/arrow-sql-server/compare/v0.1.4...v0.1.5) - 2026-07-03

### Fixed

- support Arrow view representations for writes

### Other

- gate release-plz publish job
