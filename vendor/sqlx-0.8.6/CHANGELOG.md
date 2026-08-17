# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.8.6 - 2025-05-19

9 pull requests were merged this release cycle.

### Added
* [[#3849]]: Add color and wrapping to cli help text [[@joshka]]

### Changed
* [[#3830]]: build: drop unused `tempfile` dependency [[@paolobarbolini]]
* [[#3845]]: chore: clean up no longer used imports [[@tisonkun]]
* [[#3863]]: Use unnamed statement in pg when not persistent [[@ThomWright]]
* [[#3866]]: chore(doc): clarify compile-time verification and case conversion behavior [[@duhby]]

### Fixed
* [[#3840]]: Fix docs.rs build of sqlx-sqlite [[@gferon]]
* [[#3848]]: fix(macros): don't mutate environment variables [[@joeydewaal]]
* [[#3855]]: fix `attrubute` typo in doc [[@kujeger]]
* [[#3856]]: fix(macros): slightly improve unsupported type error message [[@dyc3]]

[#3830]: https://github.com/launchbadge/sqlx/pull/3830
[#3840]: https://github.com/launchbadge/sqlx/pull/3840
[#3845]: https://github.com/launchbadge/sqlx/pull/3845
[#3848]: https://github.com/launchbadge/sqlx/pull/3848
[#3849]: https://github.com/launchbadge/sqlx/pull/3849
[#3855]: https://github.com/launchbadge/sqlx/pull/3855
[#3856]: https://github.com/launchbadge/sqlx/pull/3856
[#3863]: https://github.com/launchbadge/sqlx/pull/3863
[#3866]: https://github.com/launchbadge/sqlx/pull/3866

## 0.8.5 - 2025-04-14

Hotfix release to address two new issues:
* [[#3823]]: `sqlx-cli@0.8.4` broke `.env` default resolution mechanism
* [[#3825]]: `sqlx@0.8.4` broke test fixture setup

The `0.8.4` release will be yanked as of publishing this one.

### Added
* In release PR: `sqlx-cli` now accepts `--no-dotenv` in subcommand arguments.
* In release PR: added functionality tests for `sqlx-cli` to CI.
* In release PR: test `#[sqlx::test]` twice in CI to cover cleanup.

### Fixed
* In release PR: `sqlx-cli` correctly reads `.env` files by default again.
  * Addresses [[#3823]].
* In release PR: fix bugs in MySQL implementation of `#[sqlx::test]`.
  * Addresses [[#3825]].

[#3823]: https://github.com/launchbadge/sqlx/issues/3823
[#3825]: https://github.com/launchbadge/sqlx/issues/3825

## 0.8.4 - 2025-04-13

50 pull requests were merged this release cycle.

### Added
* [[#3603]]: Added missing special casing for encoding embedded arrays of custom types [[@nico-incubiq]]
* [[#3625]]: feat(sqlite): add preupdate hook [[@aschey]]
* [[#3655]]: docs: add example for postgres enums with type TEXT [[@tisonkun]]
* [[#3677]]: Add json(nullable) macro attribute [[@seanaye]]
* [[#3687]]: Derive clone and debug for postgresql arguments [[@remysaissy]]
* [[#3690]]: feat: add postres geometry line segment [[@jayy-lmao]]
* [[#3707]]: feat(Sqlite): add LockedSqliteHandle::last_error [[@joeydewaal]]
* [[#3710]]: feat: add ipnet support [[@BeauGieskens]]
* [[#3711]]: feat(postgres): add geometry box [[@jayy-lmao]]
* [[#3714]]: chore: expose bstr feature [[@joeydewaal]]
* [[#3716]]: feat(postgres): add geometry path [[@jayy-lmao]]
* [[#3724]]: feat(sqlx-cli): Add flag to disable automatic loading of .env files [[@benwilber]]
* [[#3734]]: QueryBuilder: add debug_assert when `push_values` is passed an empty set of tuples [[@chanmaoganda]]
* [[#3745]]: feat: sqlx sqlite expose de/serialize [[@mattrighetti]]
* [[#3765]]: Merge of #3427 (by @mpyw) and #3614 (by @bonsairobo) [[@abonander]]
    * [[#3427]] Expose `transaction_depth` through `get_transaction_depth()` method [[@mpyw]]
      * Changed to `Connection::is_in_transaction` in [[#3765]]
    * [[#3614]] Add `begin_with` methods to support database-specific transaction options [[@bonsairobo]]
* [[#3769]]: feat(postgres): add geometry polygon [[@jayy-lmao]]
* [[#3773]]: feat(postgres): add geometry circle [[@jayy-lmao]]

### Changed
* [[#3665]]: build(deps): bump semver compatible dependencies [[@paolobarbolini]]
* [[#3669]]: refactor(cli): replace promptly with dialoguer [[@paolobarbolini]]
* [[#3672]]: add `#[track_caller]` to `Row::get()` [[@karambarakat]]
* [[#3708]]: chore(MySql): Remove unnecessary box [[@joeydewaal]]
* [[#3715]]: chore: add pg_copy regression tests [[@joeydewaal]]
* [[#3721]]: Replace some `futures-core` / `futures-util` APIs with `std` variants [[@paolobarbolini]]
* [[#3725]]: chore: replace rustls-pemfile with rustls-pki-types [[@tottoto]]
* [[#3754]]: chore(cli): remove unused async-trait crate from dependencies [[@tottoto]]
* [[#3762]]: docs(pool): recommend actix-web ThinData over Data to avoid two Arcs [[@jonasmalacofilho]]

### Fixed
* [[#3289]]: Always set `SQLITE_OPEN_URI` on in-memory sqlite [[@LecrisUT]]
* [[#3334]]: Fix: nextest cleanup race condition [[@bonega]]
* [[#3666]]: fix(cli): running tests on 32bit platforms [[@paolobarbolini]]
* [[#3686]]: fix: handle nullable values by printing NULL instead of panicking [[@joeydewaal]]
* [[#3700]]: fix(Sqlite): stop sending rows after first error [[@joeydewaal]]
* [[#3701]]: fix(postgres) use signed int for length prefix in `PgCopyIn` [[@joeydewaal]]
* [[#3703]]: fix(Postgres) chunk pg_copy data [[@joeydewaal]]
* [[#3712]]: FromRow: Fix documentation order [[@Turbo87]]
* [[#3720]]: Fix readme: uuid feature is gating for all repos [[@jthacker]]
* [[#3728]]: postgres: Fix tracing span when dropping PgListener [[@chitoku-k]]
* [[#3741]]: Fix example calculation in docs [[@dns2utf8]]
* [[#3749]]: docs: add some missing backticks [[@soulwa]]
* [[#3753]]: Avoid privilege requirements by using an advisory lock in test setup (postgres). [[@kildrens]]
* [[#3755]]: Fix FromRow docs for tuples [[@xvapx]]
* [[#3768]]: chore(Sqlite): remove ci.db from repo [[@joeydewaal]]
* [[#3771]]: fix(ci): breakage from Rustup 1.28 [[@abonander]]
* [[#3786]]: Fix a copy-paste error on get_username docs [[@sulami]]
* [[#3801]]: Fix: Enable Json type when db feature isn't enabled [[@thriller08]]
* [[#3809]]: fix: PgConnectOptions docs [[@mbj]]
* [[#3811]]: Fix error message typo in PgPoint::from_str [[@TeCHiScy]]
* [[#3812]]: mysql: Fix panic on invalid text row length field [[@0xdeafbeef]]
* [[#3815]]: fix(macros): cache macro metadata based on `CARGO_MANIFEST_DIR` [[@joeydewaal]]
* Fixes in release PR [[#3819]] [[@abonander]]:
  * fix(postgres): send `limit: 0` for all `Execute` messages
    * Addresses [[#3673]]: Parallel workers not used on Postgres 
  * fix: let `CertificateInput::from` infer any PEM-encoded document
    * Fixes `PGSSLKEY` not being parsed correctly when containing a PEM-encoded private key.
  * doc: improve documentation of `PgConnectOptions`
    * `PGHOSTADDR` now can be used to override `PGHOST`. 
    * Addresses [[#3740]]: Document the URL syntax for Unix-domain sockets when connecting to postgres

[#3819]: https://github.com/launchbadge/sqlx/pull/3819

[#3673]: https://github.com/launchbadge/sqlx/issues/3673
[#3740]: https://github.com/launchbadge/sqlx/issues/3740

[#3289]: https://github.com/launchbadge/sqlx/pull/3289
[#3334]: https://github.com/launchbadge/sqlx/pull/3334
[#3427]: https://github.com/launchbadge/sqlx/pull/3427
[#3603]: https://github.com/launchbadge/sqlx/pull/3603
[#3614]: https://github.com/launchbadge/sqlx/pull/3614
[#3625]: https://github.com/launchbadge/sqlx/pull/3625
[#3655]: https://github.com/launchbadge/sqlx/pull/3655
[#3665]: https://github.com/launchbadge/sqlx/pull/3665
[#3666]: https://github.com/launchbadge/sqlx/pull/3666
[#3669]: https://github.com/launchbadge/sqlx/pull/3669
[#3672]: https://github.com/launchbadge/sqlx/pull/3672
[#3677]: https://github.com/launchbadge/sqlx/pull/3677
[#3686]: https://github.com/launchbadge/sqlx/pull/3686
[#3687]: https://github.com/launchbadge/sqlx/pull/3687
[#3690]: https://github.com/launchbadge/sqlx/pull/3690
[#3700]: https://github.com/launchbadge/sqlx/pull/3700
[#3701]: https://github.com/launchbadge/sqlx/pull/3701
[#3703]: https://github.com/launchbadge/sqlx/pull/3703
[#3707]: https://github.com/launchbadge/sqlx/pull/3707
[#3708]: https://github.com/launchbadge/sqlx/pull/3708
[#3710]: https://github.com/launchbadge/sqlx/pull/3710
[#3711]: https://github.com/launchbadge/sqlx/pull/3711
[#3712]: https://github.com/launchbadge/sqlx/pull/3712
[#3714]: https://github.com/launchbadge/sqlx/pull/3714
[#3715]: https://github.com/launchbadge/sqlx/pull/3715
[#3716]: https://github.com/launchbadge/sqlx/pull/3716
[#3720]: https://github.com/launchbadge/sqlx/pull/3720
[#3721]: https://github.com/launchbadge/sqlx/pull/3721
[#3724]: https://github.com/launchbadge/sqlx/pull/3724
[#3725]: https://github.com/launchbadge/sqlx/pull/3725
[#3728]: https://github.com/launchbadge/sqlx/pull/3728
[#3734]: https://github.com/launchbadge/sqlx/pull/3734
[#3741]: https://github.com/launchbadge/sqlx/pull/3741
[#3745]: https://github.com/launchbadge/sqlx/pull/3745
[#3749]: https://github.com/launchbadge/sqlx/pull/3749
[#3753]: https://github.com/launchbadge/sqlx/pull/3753
[#3754]: https://github.com/launchbadge/sqlx/pull/3754
[#3755]: https://github.com/launchbadge/sqlx/pull/3755
[#3762]: https://github.com/launchbadge/sqlx/pull/3762
[#3765]: https://github.com/launchbadge/sqlx/pull/3765
[#3768]: https://github.com/launchbadge/sqlx/pull/3768
[#3769]: https://github.com/launchbadge/sqlx/pull/3769
[#3771]: https://github.com/launchbadge/sqlx/pull/3771
[#3773]: https://github.com/launchbadge/sqlx/pull/3773
[#3786]: https://github.com/launchbadge/sqlx/pull/3786
[#3801]: https://github.com/launchbadge/sqlx/pull/3801
[#3809]: https://github.com/launchbadge/sqlx/pull/3809
[#3811]: https://github.com/launchbadge/sqlx/pull/3811
[#3812]: https://github.com/launchbadge/sqlx/pull/3812
[#3815]: https://github.com/launchbadge/sqlx/pull/3815

## 0.8.3 - 2025-01-03

41 pull requests were merged this release cycle.

### Added
* [[#3418]]: parse timezone parameter in mysql connection url [[@dojiong]]
* [[#3491]]: chore: Update async-std v1.13 [[@jayvdb]]
* [[#3492]]: expose relation_id and relation_attribution_no on PgColumn [[@kurtbuilds]]
* [[#3493]]: doc(sqlite): document behavior for zoned date-time types [[@abonander]]
* [[#3500]]: Add sqlite commit and rollback hooks [[@gridbox]]
* [[#3505]]: chore(mysql): create test for passwordless auth (#3484) [[@abonander]]
* [[#3507]]: Add a "sqlite-unbundled" feature that dynamically links to system libsqlite3.so library [[@lilydjwg]]
* [[#3508]]: doc(sqlite): show how to turn options into a pool [[@M3t0r]]
* [[#3514]]: Support PgHstore by default in macros [[@joeydewaal]]
* [[#3550]]: Implement Acquire for PgListener [[@sandhose]]
* [[#3551]]: Support building with rustls but native certificates [[@IlyaBizyaev]]
* [[#3553]]: Add support for Postgres lquery arrays [[@philipcristiano]]
* [[#3560]]: Add PgListener::next_buffered(), to support batch processing of notifications [[@chanks]]
* [[#3577]]: Derive Copy where possible for database-specific types [[@veigaribo]]
* [[#3579]]: Reexport AnyTypeInfoKind [[@Norlock]]
* [[#3580]]: doc(mysql): document difference between `Uuid` and `uuid::fmt::Hyphenated` [[@abonander]]
* [[#3583]]: feat: point [[@jayy-lmao]]
* [[#3608]]: Implement AnyQueryResult for Sqlite and MySQL [[@pxp9]]
* [[#3623]]: feat: add geometry line [[@jayy-lmao]]
* [[#3658]]: feat: add Transaction type aliases [[@joeydewaal]]

### Changed
* [[#3519]]: Remove unused dependencies from sqlx-core, sqlx-cli and sqlx-postgres [[@vsuryamurthy]]
* [[#3529]]: Box Pgconnection fields [[@joeydewaal]]
* [[#3548]]: Demote `.pgpass` file warning to a debug message. [[@denschub]]
* [[#3585]]: Eagerly reconnect in `PgListener::try_recv` [[@swlynch99]]
* [[#3596]]: Bump thiserror to v2.0.0 [[@paolobarbolini]]
* [[#3605]]: Use `UNION ALL` instead of `UNION` in nullable check [[@Suficio]]
* [[#3629]]: chore: remove BoxFuture's (non-breaking) [[@joeydewaal]]
* [[#3632]]: Bump hashlink to v0.10 [[@paolobarbolini]]
* [[#3643]]: Roll PostgreSQL 11..=15 tests to 13..=17 [[@paolobarbolini]]
* [[#3648]]: close listener connection on TimedOut and BrokenPipe errors [[@DXist]]
* [[#3649]]: Bump hashbrown to v0.15 [[@paolobarbolini]]

### Fixed
* [[#3528]]: fix: obey `no-transaction` flag in down migrations [[@manifest]]
* [[#3536]]: fix: using sqlx::test macro inside macro's [[@joeydewaal]]
* [[#3545]]: fix: remove `sqlformat` [[@tbar4]]
* [[#3558]]: fix: fix example code of `query_as` [[@xuehaonan27]]
* [[#3566]]: Fix: Cannot query Postgres `INTERVAL[]` [[@Ddystopia]]
* [[#3593]]: fix: URL decode database name when parsing connection url [[@BenoitRanque]]
* [[#3601]]: Remove default-features = false from url [[@hsivonen]]
* [[#3604]]: Fix mistake in sqlx::test fixtures docs [[@andreweggleston]]
* [[#3612]]: fix(mysql): percent-decode database name [[@abonander]]
* [[#3640]]: Dont use `EXPLAIN` in nullability check for QuestDB [[@Suficio]]

[#3418]: https://github.com/launchbadge/sqlx/pull/3418
[#3478]: https://github.com/launchbadge/sqlx/pull/3478
[#3491]: https://github.com/launchbadge/sqlx/pull/3491
[#3492]: https://github.com/launchbadge/sqlx/pull/3492
[#3493]: https://github.com/launchbadge/sqlx/pull/3493
[#3500]: https://github.com/launchbadge/sqlx/pull/3500
[#3505]: https://github.com/launchbadge/sqlx/pull/3505
[#3507]: https://github.com/launchbadge/sqlx/pull/3507
[#3508]: https://github.com/launchbadge/sqlx/pull/3508
[#3514]: https://github.com/launchbadge/sqlx/pull/3514
[#3519]: https://github.com/launchbadge/sqlx/pull/3519
[#3528]: https://github.com/launchbadge/sqlx/pull/3528
[#3529]: https://github.com/launchbadge/sqlx/pull/3529
[#3536]: https://github.com/launchbadge/sqlx/pull/3536
[#3545]: https://github.com/launchbadge/sqlx/pull/3545
[#3548]: https://github.com/launchbadge/sqlx/pull/3548
[#3550]: https://github.com/launchbadge/sqlx/pull/3550
[#3551]: https://github.com/launchbadge/sqlx/pull/3551
[#3553]: https://github.com/launchbadge/sqlx/pull/3553
[#3558]: https://github.com/launchbadge/sqlx/pull/3558
[#3560]: https://github.com/launchbadge/sqlx/pull/3560
[#3566]: https://github.com/launchbadge/sqlx/pull/3566
[#3577]: https://github.com/launchbadge/sqlx/pull/3577
[#3579]: https://github.com/launchbadge/sqlx/pull/3579
[#3580]: https://github.com/launchbadge/sqlx/pull/3580
[#3583]: https://github.com/launchbadge/sqlx/pull/3583
[#3585]: https://github.com/launchbadge/sqlx/pull/3585
[#3593]: https://github.com/launchbadge/sqlx/pull/3593
[#3596]: https://github.com/launchbadge/sqlx/pull/3596
[#3601]: https://github.com/launchbadge/sqlx/pull/3601
[#3604]: https://github.com/launchbadge/sqlx/pull/3604
[#3605]: https://github.com/launchbadge/sqlx/pull/3605
[#3608]: https://github.com/launchbadge/sqlx/pull/3608
[#3612]: https://github.com/launchbadge/sqlx/pull/3612
[#3623]: https://github.com/launchbadge/sqlx/pull/3623
[#3629]: https://github.com/launchbadge/sqlx/pull/3629
[#3632]: https://github.com/launchbadge/sqlx/pull/3632
[#3640]: https://github.com/launchbadge/sqlx/pull/3640
[#3643]: https://github.com/launchbadge/sqlx/pull/3643
[#3648]: https://github.com/launchbadge/sqlx/pull/3648
[#3649]: https://github.com/launchbadge/sqlx/pull/3649
[#3658]: https://github.com/launchbadge/sqlx/pull/3658


## 0.8.2 - 2024-09-02

10 pull requests were merged this release cycle.

This release addresses a few regressions that have occurred, and refines SQLx's MSRV policy (see [the FAQ](FAQ.md)).

### Added
* [[#3447]]: Clarify usage of Json/Jsonb in query macros [[@Lachstec]]

### Changed
* [[#3424]]: Remove deprecated feature-names from `Cargo.toml` files in examples [[@carschandler]]

### Fixed
* [[#3403]]: Fix (#3395) sqlx::test macro in 0.8 [[@joeydewaal]]
* [[#3411]]: fix: Use rfc3339 to decode date from text [[@pierre-wehbe]]
* [[#3453]]: fix(#3445): PgHasArrayType [[@joeydewaal]]
    * Fixes `#[sqlx(no_pg_array)]` being forbidden on `#[derive(Type)]` structs. 
* [[#3454]]: fix: non snake case warning [[@joeydewaal]]
* [[#3459]]: Pgsql cube type compile fail [[@kdesjard]]
* [[#3465]]: fix(postgres): max number of binds is 65535, not 32767 (regression) [[@abonander]]
* [[#3467]]: fix cancellation issues with `PgListener`, `PgStream::recv()` [[@abonander]]
    * Fixes cryptic `unknown message: "\\0"` error 
* [[#3474]]: Fix try_get example in README.md [[@luveti]]

[#3403]: https://github.com/launchbadge/sqlx/pull/3403
[#3411]: https://github.com/launchbadge/sqlx/pull/3411
[#3424]: https://github.com/launchbadge/sqlx/pull/3424
[#3447]: https://github.com/launchbadge/sqlx/pull/3447
[#3453]: https://github.com/launchbadge/sqlx/pull/3453
[#3454]: https://github.com/launchbadge/sqlx/pull/3454
[#3455]: https://github.com/launchbadge/sqlx/pull/3455
[#3459]: https://github.com/launchbadge/sqlx/pull/3459
[#3465]: https://github.com/launchbadge/sqlx/pull/3465
[#3467]: https://github.com/launchbadge/sqlx/pull/3467
[#3474]: https://github.com/launchbadge/sqlx/pull/3474

## 0.8.1 - 2024-08-23

16 pull requests were merged this release cycle.

This release contains a fix for [RUSTSEC-2024-0363]. 

Postgres users are advised to upgrade ASAP as a possible exploit has been demonstrated:
<https://github.com/launchbadge/sqlx/issues/3440#issuecomment-2307956901>

MySQL and SQLite do not _appear_ to be exploitable, but upgrading is recommended nonetheless.

### Added
* [[#3421]]: correct spelling of `MySqlConnectOptions::no_engine_substitution()` [[@kolinfluence]]
    * Deprecates `MySqlConnectOptions::no_engine_subsitution()` (oops) in favor of the correctly spelled version.

### Changed
* [[#3376]]: doc: hide `spec_error` module [[@abonander]]
    * This is a helper module for the macros and was not meant to be exposed.
    * It is not expected to receive any breaking changes for the 0.8.x release, but is not designed as a public API.
      Use at your own risk.
* [[#3382]]: feat: bumped to `libsqlite3-sys=0.30.1` to support sqlite 3.46 [[@CommanderStorm]]
* [[#3385]]: chore(examples):Migrated the pg-chat example to ratatui [[@CommanderStorm]]
* [[#3399]]: Upgrade to rustls 0.23 [[@djc]]
    * RusTLS now has pluggable cryptography providers: `ring` (the existing implementation),
      and `aws-lc-rs` which has optional FIPS certification.
    * The existing features activating RusTLS (`runtime-tokio-rustls`, `runtime-async-std-rustls`, `tls-rustls`)
      enable the `ring` provider of RusTLS to match the existing behavior so this _should not_ be a breaking change. 
    * Switch to the `tls-rustls-aws-lc-rs` feature to use the `aws-lc-rs` provider.
      * If using `runtime-tokio-rustls` or `runtime-async-std-rustls`, 
        this will necessitate switching to the appropriate non-legacy runtime feature: 
        `runtime-tokio` or `runtime-async-std`
    * See the RusTLS README for more details: <https://github.com/rustls/rustls?tab=readme-ov-file#cryptography-providers>

### Fixed
* [[#2786]]: fix(sqlx-cli): do not clean sqlx during prepare [[@cycraig]]
* [[#3354]]: sqlite: fix inconsistent read-after-write [[@ckampfe]]
* [[#3371]]: Fix encoding and decoding of MySQL enums in `sqlx::Type` [[@alu]]
* [[#3374]]: fix: usage of `node12` in `SQLx` action [[@hamirmahal]]
* [[#3380]]: chore: replace structopt with clap in examples [[@tottoto]]
* [[#3381]]: Fix CI after Rust 1.80, remove dead feature references [[@abonander]]
* [[#3384]]: chore(tests): fixed deprecation warnings [[@CommanderStorm]]
* [[#3386]]: fix(dependencys):bumped cargo_metadata to `v0.18.1` to avoid yanked `v0.14.3` [[@CommanderStorm]]
* [[#3389]]: fix(cli): typo in error for required DB URL [[@ods]]
* [[#3417]]: Update version to 0.8 in README [[@soucosmo]]
* [[#3441]]: fix: audit protocol handling [[@abonander]]
    * This addresses [RUSTSEC-2024-0363] and includes regression tests for MySQL, Postgres and SQLite.

[#2786]: https://github.com/launchbadge/sqlx/pull/2786
[#3354]: https://github.com/launchbadge/sqlx/pull/3354
[#3371]: https://github.com/launchbadge/sqlx/pull/3371
[#3374]: https://github.com/launchbadge/sqlx/pull/3374
[#3376]: https://github.com/launchbadge/sqlx/pull/3376
[#3380]: https://github.com/launchbadge/sqlx/pull/3380
[#3381]: https://github.com/launchbadge/sqlx/pull/3381
[#3382]: https://github.com/launchbadge/sqlx/pull/3382
[#3384]: https://github.com/launchbadge/sqlx/pull/3384
[#3385]: https://github.com/launchbadge/sqlx/pull/3385
[#3386]: https://github.com/launchbadge/sqlx/pull/3386
[#3389]: https://github.com/launchbadge/sqlx/pull/3389
[#3399]: https://github.com/launchbadge/sqlx/pull/3399
[#3417]: https://github.com/launchbadge/sqlx/pull/3417
[#3421]: https://github.com/launchbadge/sqlx/pull/3421
[#3441]: https://github.com/launchbadge/sqlx/pull/3441

[RUSTSEC-2024-0363]: https://rustsec.org/advisories/RUSTSEC-2024-0363.html

## 0.8.0 - 2024-07-22

70 pull requests were merged this release cycle.

[#2697] was merged the same day as release 0.7.4 and so was missed by the automatic CHANGELOG generation.

### Breaking
* [[#2697]]: fix(macros): only enable chrono when time is disabled [[@saiintbrisson]]
* [[#2973]]: Generic Associated Types in Database, replacing HasValueRef, HasArguments, HasStatement [[@nitn3lav]]
* [[#2482]]: chore: bump syn to 2.0 [[@saiintbrisson]]
    * Deprecated type ascription syntax in the query macros was removed.
* [[#2736]]: Fix describe on PostgreSQL views with rules [[@tsing]]
  * Potentially breaking: nullability inference changes for Postgres.
* [[#2869]]: Implement PgHasArrayType for all references [[@tylerhawkes]]
    * Conflicts with existing manual implementations.
* [[#2940]]: fix: Decode and Encode derives (#1031) [[@benluelo]]
    * Changes lifetime obligations for field types.
* [[#3064]]: Sqlite explain graph [[@tyrelr]]
    * Potentially breaking: nullability inference changes for SQLite.
* [[#3123]]: Reorder attrs in sqlx::test macro [[@bobozaur]]
    * Potentially breaking: attributes on `#[sqlx::test]` usages are applied in the correct order now.
* [[#3126]]: Make Encode return a result [[@FSMaxB]]
* [[#3130]]: Add version information for failed cli migration (#3129) [[@FlakM]]
    * Breaking changes to `MigrateError`.
* [[#3181]]: feat: no tx migration [[@cleverjam]]
    * (Postgres only) migrations that should not run in a transaction can be flagged by adding `-- no-transaction` to the beginning.
    * Breaking change: added field to `Migration`
* [[#3184]]: [BREAKING} fix(sqlite): always use `i64` as intermediate when decoding [[@abonander]]
    * integer decoding will now loudly error on overflow instead of silently truncating.
    * some usages of the query!() macros might change an i32 to an i64.
* [[#3252]]: fix `#[derive(sqlx::Type)]` in Postgres [[@abonander]]
  * Manual implementations of PgHasArrayType for enums will conflict with the generated one. Delete the manual impl or add `#[sqlx(no_pg_array)]` where conflicts occur.
  * Type equality for PgTypeInfo is now schema-aware.
* [[#3329]]: fix: correct handling of arrays of custom types in Postgres [[@abonander]]
    * Potential breaking change: `PgTypeInfo::with_name()` infers types that start with `_` to be arrays of the un-prefixed type. Wrap type names in quotes to bypass this behavior.
* [[#3356]]: breaking: fix name collision in `FromRow`, return `Error::ColumnDecode` for `TryFrom` errors [[@abonander]]
    * Breaking behavior change: errors with `#[sqlx(try_from = "T")]` now return `Error::ColumnDecode` instead of `Error::ColumnNotFound`.
    * Breaking because `#[sqlx(default)]` on an individual field or the struct itself would have previously suppressed the error. 
      This doesn't seem like good behavior as it could result in some potentially very difficult bugs.
      * Instead, create a wrapper implementing `From` and apply the default explicitly.
* [[#3337]]: allow rename with rename_all (close #2896) [[@DirectorX]]
    * Changes the precedence of `#[sqlx(rename)]` and `#[sqlx(rename_all)]` to match the expected behavior (`rename` wins).
* [[#3285]]: fix: use correct names for sslmode options [[@lily-mosquitoes]]
    * Changes the output of `ConnectOptions::to_url_lossy()` to match what parsing expects.

### Added
* [[#2917]]: Add Debug impl for PgRow [[@g-bartoszek]]
* [[#3113]]: feat: new derive feature flag [[@saiintbrisson]]
* [[#3154]]: feat: add `MySqlTime`, audit `mysql::types` for panics [[@abonander]]
* [[#3188]]: feat(cube): support postgres cube [[@jayy-lmao]]
* [[#3244]]: feat: support `NonZero*` scalar types [[@AlphaKeks]]
* [[#3260]]: feat: Add set_update_hook on SqliteConnection [[@gridbox]]
* [[#3291]]: feat: support the Postgres Bool type for the Any driver [[@etorreborre]]
* [[#3293]]: Add LICENSE-* files to crates [[@LecrisUT]]
* [[#3303]]: add array support for NonZeroI* in postgres [[@JohannesIBK]]
* [[#3311]]: Add example on how to use Transaction as Executor [[@Lachstec]]
* [[#3343]]: Add support for PostgreSQL HSTORE data type [[@KobusEllis]]

### Changed
* [[#2652]]: MySQL: Remove collation compatibility check for strings [[@alu]]
* [[#2960]]: Removed `Send` trait bound from argument binding [[@bobozaur]]
* [[#2970]]: refactor: lift type mappings into driver crates [[@abonander]]
* [[#3148]]: Bump libsqlite3-sys to v0.28 [[@NfNitLoop]]
    * Note: version bumps to `libsqlite3-sys` are not considered breaking changes as per our semver guarantees.
* [[#3265]]: perf: box `MySqlConnection` to reduce sizes of futures [[@stepantubanov]]
* [[#3352]]: chore:added a testcase for `sqlx migrate add ...` [[@CommanderStorm]]
* [[#3340]]: ci: Add job to check that sqlx builds with its declared minimum dependencies [[@iamjpotts]]

### Fixed
* [[#2702]]: Constrain cyclic associated types to themselves [[@BadBastion]]
* [[#2954]]: Fix several inter doc links [[@ralpha]]
* [[#3073]]: feat(logging): Log slow acquires from connection pool [[@iamjpotts]]
* [[#3137]]: SqliteConnectOptions::filename() memory fix (#3136) [[@hoxxep]]
* [[#3138]]: PostgreSQL Bugfix: Ensure connection is usable after failed COPY inside a transaction [[@feikesteenbergen]]
* [[#3146]]: fix(sqlite): delete unused `ConnectionHandleRaw` type [[@abonander]]
* [[#3162]]: Drop urlencoding dependency [[@paolobarbolini]]
* [[#3165]]: Bump deps that do not need code changes [[@GnomedDev]]
* [[#3167]]: fix(ci): use `docker compose` instead of `docker-compose` [[@abonander]]
* [[#3172]]: fix: Option decoding in any driver [[@pxp9]]
* [[#3173]]: fix(postgres) : int type conversion while decoding [[@RaghavRox]]
* [[#3190]]: Update time to 0.3.36 [[@BlackSoulHub]]
* [[#3191]]: Fix unclean TLS shutdown [[@levkk]]
* [[#3194]]: Fix leaking connections in fetch_optional (#2647) [[@danjpgriffin]]
* [[#3216]]: security: bump rustls to 0.21.11 [[@toxeus]]
* [[#3230]]: fix: sqlite pragma order for auto_vacuum [[@jasonish]]
* [[#3233]]: fix: get_filename should not consume self [[@jasonish]]
* [[#3234]]: fix(ci): pin Rust version, ditch unmaintained actions [[@abonander]]
* [[#3236]]: fix: resolve `path` ownership problems when using `sqlx_macros_unstable` [[@lily-mosquitoes]]
* [[#3254]]: fix: hide `sqlx_postgres::any` [[@Zarathustra2]]
* [[#3266]]: ci: MariaDB - add back 11.4 and add 11.5 [[@grooverdan]]
* [[#3267]]: ci: syntax fix [[@grooverdan]]
* [[#3271]]: docs(sqlite): fix typo - unixtime() -> unixepoch() [[@joelkoen]]
* [[#3276]]: Invert boolean for `migrate` error message. (#3275) [[@nk9]]
* [[#3279]]: fix Clippy errors [[@abonander]]
* [[#3288]]: fix: sqlite update_hook char types [[@jasonish]]
* [[#3297]]: Pass the `persistent` query setting when preparing queries with the `Any` driver [[@etorreborre]]
* [[#3298]]: Track null arguments in order to provide the appropriate type when converting them. [[@etorreborre]]
* [[#3312]]: doc: Minor rust docs fixes [[@SrGesus]]
* [[#3327]]: chore: fixed one usage of `select_input_type!()` being unhygenic [[@CommanderStorm]]
* [[#3328]]: fix(ci): comment not separated from other characters [[@hamirmahal]]
* [[#3341]]: refactor: Resolve cargo check warnings in postgres examples [[@iamjpotts]]
* [[#3346]]: fix(postgres): don't panic if `M` or `C` Notice fields are not UTF-8 [[@YgorSouza]]
* [[#3350]]: fix:the `json`-feature should activate `sqlx-postgres?/json` as well [[@CommanderStorm]]
* [[#3353]]: fix: build script new line at eof [[@Zarthus]]
* (no PR): activate `clock` and `std` features of `workspace.dependencies.chrono`.

[#2482]: https://github.com/launchbadge/sqlx/pull/2482
[#2652]: https://github.com/launchbadge/sqlx/pull/2652
[#2697]: https://github.com/launchbadge/sqlx/pull/2697
[#2702]: https://github.com/launchbadge/sqlx/pull/2702
[#2736]: https://github.com/launchbadge/sqlx/pull/2736
[#2869]: https://github.com/launchbadge/sqlx/pull/2869
[#2917]: https://github.com/launchbadge/sqlx/pull/2917
[#2940]: https://github.com/launchbadge/sqlx/pull/2940
[#2954]: https://github.com/launchbadge/sqlx/pull/2954
[#2960]: https://github.com/launchbadge/sqlx/pull/2960
[#2970]: https://github.com/launchbadge/sqlx/pull/2970
[#2973]: https://github.com/launchbadge/sqlx/pull/2973
[#3064]: https://github.com/launchbadge/sqlx/pull/3064
[#3073]: https://github.com/launchbadge/sqlx/pull/3073
[#3113]: https://github.com/launchbadge/sqlx/pull/3113
[#3123]: https://github.com/launchbadge/sqlx/pull/3123
[#3126]: https://github.com/launchbadge/sqlx/pull/3126
[#3130]: https://github.com/launchbadge/sqlx/pull/3130
[#3137]: https://github.com/launchbadge/sqlx/pull/3137
[#3138]: https://github.com/launchbadge/sqlx/pull/3138
[#3146]: https://github.com/launchbadge/sqlx/pull/3146
[#3148]: https://github.com/launchbadge/sqlx/pull/3148
[#3154]: https://github.com/launchbadge/sqlx/pull/3154
[#3162]: https://github.com/launchbadge/sqlx/pull/3162
[#3165]: https://github.com/launchbadge/sqlx/pull/3165
[#3167]: https://github.com/launchbadge/sqlx/pull/3167
[#3172]: https://github.com/launchbadge/sqlx/pull/3172
[#3173]: https://github.com/launchbadge/sqlx/pull/3173
[#3181]: https://github.com/launchbadge/sqlx/pull/3181
[#3184]: https://github.com/launchbadge/sqlx/pull/3184
[#3188]: https://github.com/launchbadge/sqlx/pull/3188
[#3190]: https://github.com/launchbadge/sqlx/pull/3190
[#3191]: https://github.com/launchbadge/sqlx/pull/3191
[#3194]: https://github.com/launchbadge/sqlx/pull/3194
[#3216]: https://github.com/launchbadge/sqlx/pull/3216
[#3230]: https://github.com/launchbadge/sqlx/pull/3230
[#3233]: https://github.com/launchbadge/sqlx/pull/3233
[#3234]: https://github.com/launchbadge/sqlx/pull/3234
[#3236]: https://github.com/launchbadge/sqlx/pull/3236
[#3244]: https://github.com/launchbadge/sqlx/pull/3244
[#3252]: https://github.com/launchbadge/sqlx/pull/3252
[#3254]: https://github.com/launchbadge/sqlx/pull/3254
[#3260]: https://github.com/launchbadge/sqlx/pull/3260
[#3265]: https://github.com/launchbadge/sqlx/pull/3265
[#3266]: https://github.com/launchbadge/sqlx/pull/3266
[#3267]: https://github.com/launchbadge/sqlx/pull/3267
[#3271]: https://github.com/launchbadge/sqlx/pull/3271
[#3276]: https://github.com/launchbadge/sqlx/pull/3276
[#3279]: https://github.com/launchbadge/sqlx/pull/3279
[#3285]: https://github.com/launchbadge/sqlx/pull/3285
[#3288]: https://github.com/launchbadge/sqlx/pull/3288
[#3291]: https://github.com/launchbadge/sqlx/pull/3291
[#3293]: https://github.com/launchbadge/sqlx/pull/3293
[#3297]: https://github.com/launchbadge/sqlx/pull/3297
[#3298]: https://github.com/launchbadge/sqlx/pull/3298
[#3303]: https://github.com/launchbadge/sqlx/pull/3303
[#3311]: https://github.com/launchbadge/sqlx/pull/3311
[#3312]: https://github.com/launchbadge/sqlx/pull/3312
[#3327]: https://github.com/launchbadge/sqlx/pull/3327
[#3328]: https://github.com/launchbadge/sqlx/pull/3328
[#3329]: https://github.com/launchbadge/sqlx/pull/3329
[#3337]: https://github.com/launchbadge/sqlx/pull/3337
[#3340]: https://github.com/launchbadge/sqlx/pull/3340
[#3341]: https://github.com/launchbadge/sqlx/pull/3341
[#3343]: https://github.com/launchbadge/sqlx/pull/3343
[#3346]: https://github.com/launchbadge/sqlx/pull/3346
[#3350]: https://github.com/launchbadge/sqlx/pull/3350
[#3352]: https://github.com/launchbadge/sqlx/pull/3352
[#3353]: https://github.com/launchbadge/sqlx/pull/3353
[#3356]: https://github.com/launchbadge/sqlx/pull/3356
## 0.7.4 - 2024-03-11

38 pull requests were merged this release cycle.

This is officially the **last** release of the 0.7.x release cycle.

As of this release, development of 0.8.0 has begun on `main` and only high-priority bugfixes may be backported.

### Added

* [[#2891]]: feat: expose getters for connect options fields [[@saiintbrisson]]
* [[#2902]]: feat: add `to_url_lossy` to connect options [[@lily-mosquitoes]]
* [[#2927]]: Support `query!` for cargo-free systems [[@kshramt]]
* [[#2997]]: doc(FAQ): add entry explaining prepared statements [[@abonander]]
* [[#3001]]: Update README to clarify MariaDB support [[@iangilfillan]]
* [[#3004]]: feat(logging): Add numeric elapsed time field elapsed_secs [[@iamjpotts]]
* [[#3007]]: feat: add `raw_sql` API [[@abonander]]
    * This hopefully makes it easier to find how to execute statements which are not supported by the default
      prepared statement interfaces `query*()` and `query!()`.
    * Improved documentation across the board for the `query*()` functions.
    * Deprecated: `execute_many()` and `fetch_many()` on interfaces that use prepared statements.
        * Multiple SQL statements in one query string were only supported by SQLite because its prepared statement
          interface is the *only* way to execute SQL. All other database flavors forbid multiple statements in
          one prepared statement string as an extra defense against SQL injection.
        * The new `raw_sql` API retains this functionality because it explicitly does *not* use prepared statements.
          Raw or text-mode query interfaces generally allow multiple statements in one query string, and this is
          supported by all current databases. Due to their nature, however, one cannot use bind parameters with them.
        * If this change affects you, an issue is open for discussion: https://github.com/launchbadge/sqlx/issues/3108
* [[#3011]]: Added support to IpAddr with MySQL/MariaDB. [[@Icerath]]
* [[#3013]]: Add default implementation for PgInterval [[@pawurb]]
* [[#3018]]: Add default implementation for PgMoney [[@pawurb]]
* [[#3026]]: Update docs to reflect support for MariaDB data types [[@iangilfillan]]
* [[#3037]]: feat(mysql): allow to connect with mysql driver without default behavor [[@darkecho731]]

### Changed

* [[#2900]]: Show latest url to docs for macro.migrate [[@Vrajs16]]
* [[#2914]]: Use `create_new` instead of `atomic-file-write` [[@mattfbacon]]
* [[#2926]]: docs: update example for `PgConnectOptions` [[@Fyko]]
* [[#2989]]: sqlx-core: Remove dotenvy dependency [[@joshtriplett]]
* [[#2996]]: chore: Update ahash to 0.8.7 [[@takenoko-gohan]]
* [[#3006]]: chore(deps): Replace unmaintained tempdir crate with tempfile [[@iamjpotts]]
* [[#3008]]: chore: Ignore .sqlx folder created by running ci steps locally [[@iamjpotts]]
* [[#3009]]: chore(dev-deps): Upgrade env_logger from 0.9 to 0.11 [[@iamjpotts]]
* [[#3010]]: chore(deps): Upgrade criterion to 0.5.1 [[@iamjpotts]]
* [[#3050]]: Optimize SASL auth in sqlx-postgres [[@mirek26]]
* [[#3055]]: Set TCP_NODELAY option on TCP sockets [[@mirek26]]
* [[#3065]]: Improve max_lifetime handling [[@mirek26]]
* [[#3072]]: Change the name of "inner" function generated by `#[sqlx::test]` [[@ciffelia]]
* [[#3083]]: Remove sha1 because it's not being used in postgres [[@rafaelGuerreiro]]

### Fixed

* [[#2898]]: Fixed docs [[@Vrajs16]]
* [[#2905]]: fix(mysql): Close prepared statement if persistence is disabled [[@larsschumacher]]
* [[#2913]]: Fix handling of deferred constraints [[@Thomasdezeeuw]]
* [[#2919]]: fix duplicate "`" in FromRow "default" attribute doc comment [[@shengsheng]]
* [[#2932]]: fix(postgres): avoid unnecessary flush in PgCopyIn::read_from [[@tsing]]
* [[#2955]]: Minor fixes [[@Dawsoncodes]]
* [[#2963]]: Fixed ReadMe badge styling [[@tadghh]]
* [[#2976]]: fix: AnyRow not support PgType::Varchar [[@holicc]]
* [[#3053]]: fix: do not panic when binding a large BigDecimal [[@Ekleog]]
* [[#3056]]: fix: spans in sqlite tracing (#2876) [[@zoomiti]]
* [[#3089]]: fix(migrate): improve error message when parsing version from filename [[@abonander]]
* [[#3098]]: Migrations fixes [[@abonander]]
  * Unhides `sqlx::migrate::Migrator`.
  * Improves I/O error message when failing to read a file in `migrate!()`.

[#2891]: https://github.com/launchbadge/sqlx/pull/2891
[#2898]: https://github.com/launchbadge/sqlx/pull/2898
[#2900]: https://github.com/launchbadge/sqlx/pull/2900
[#2902]: https://github.com/launchbadge/sqlx/pull/2902
[#2905]: https://github.com/launchbadge/sqlx/pull/2905
[#2913]: https://github.com/launchbadge/sqlx/pull/2913
[#2914]: https://github.com/launchbadge/sqlx/pull/2914
[#2919]: https://github.com/launchbadge/sqlx/pull/2919
[#2926]: https://github.com/launchbadge/sqlx/pull/2926
[#2927]: https://github.com/launchbadge/sqlx/pull/2927
[#2932]: https://github.com/launchbadge/sqlx/pull/2932
[#2955]: https://github.com/launchbadge/sqlx/pull/2955
[#2963]: https://github.com/launchbadge/sqlx/pull/2963
[#2976]: https://github.com/launchbadge/sqlx/pull/2976
[#2989]: https://github.com/launchbadge/sqlx/pull/2989
[#2996]: https://github.com/launchbadge/sqlx/pull/2996
[#2997]: https://github.com/launchbadge/sqlx/pull/2997
[#3001]: https://github.com/launchbadge/sqlx/pull/3001
[#3004]: https://github.com/launchbadge/sqlx/pull/3004
[#3006]: https://github.com/launchbadge/sqlx/pull/3006
[#3007]: https://github.com/launchbadge/sqlx/pull/3007
[#3008]: https://github.com/launchbadge/sqlx/pull/3008
[#3009]: https://github.com/launchbadge/sqlx/pull/3009
[#3010]: https://github.com/launchbadge/sqlx/pull/3010
[#3011]: https://github.com/launchbadge/sqlx/pull/3011
[#3013]: https://github.com/launchbadge/sqlx/pull/3013
[#3018]: https://github.com/launchbadge/sqlx/pull/3018
[#3026]: https://github.com/launchbadge/sqlx/pull/3026
[#3037]: https://github.com/launchbadge/sqlx/pull/3037
[#3050]: https://github.com/launchbadge/sqlx/pull/3050
[#3053]: https://github.com/launchbadge/sqlx/pull/3053
[#3055]: https://github.com/launchbadge/sqlx/pull/3055
[#3056]: https://github.com/launchbadge/sqlx/pull/3056
[#3065]: https://github.com/launchbadge/sqlx/pull/3065
[#3072]: https://github.com/launchbadge/sqlx/pull/3072
[#3083]: https://github.com/launchbadge/sqlx/pull/3083
[#3089]: https://github.com/launchbadge/sqlx/pull/3089
[#3098]: https://github.com/launchbadge/sqlx/pull/3098

## 0.7.3 - 2023-11-22

38 pull requests were merged this release cycle.

### Added
* [[#2478]]: feat(citext): support postgres citext [[@hgranthorner]]
* [[#2545]]: Add `fixtures_path` in sqlx::test args [[@ripa1995]]
* [[#2665]]: feat(mysql): support packet splitting [[@tk2217]]
* [[#2752]]: Enhancement #2747 Provide `fn PgConnectOptions::get_host(&self)` [[@boris-lok]]
* [[#2769]]: Customize the macro error message based on the metadata [[@Nemo157]]
* [[#2793]]: derived Hash trait for PgInterval [[@yasamoka]]
* [[#2801]]: derive FromRow: sqlx(default) for all fields [[@grgi]]
* [[#2827]]: Add impl `FromRow` for the unit type [[@nanoqsh]]
* [[#2871]]: Add `MySqlConnectOptions::get_database()`  [[@shiftrightonce]]
* [[#2873]]: Sqlx Cli: Added force flag to drop database for postgres [[@Vrajs16]]
* [[#2894]]: feat: `Text` adapter [[@abonander]]

### Changed
* [[#2701]]: Remove documentation on offline feature [[@Baptistemontan]]
* [[#2713]]: Add additional info regarding using Transaction and PoolConnection as… [[@satwanjyu]]
* [[#2770]]: Update README.md [[@snspinn]]
* [[#2797]]: doc(mysql): document behavior regarding `BOOLEAN` and the query macros [[@abonander]]
* [[#2803]]: Don't use separate temp dir for query jsons (2)  [[@mattfbacon]]
* [[#2819]]: postgres begin cancel safe [[@conradludgate]]
* [[#2832]]: Update extra_float_digits default to 2 instead of 3 [[@brianheineman]]
* [[#2865]]: Update Faq - Bulk upsert with optional fields  [[@Vrajs16]]
* [[#2880]]: feat: use specific message for slow query logs [[@abonander]]
* [[#2882]]: Do not require db url for prepare [[@tamasfe]]
* [[#2890]]: doc(sqlite): cover lack of `NUMERIC` support [[@abonander]]
* [No PR]: Upgraded `libsqlite3-sys` to 0.27.0
  * Note: linkage to `libsqlite3-sys` is considered semver-exempt; 
    see the release notes for 0.7.0 below for details.

### Fixed
* [[#2640]]: fix: sqlx::macro db cleanup race condition by adding a margin to current timestamp [[@fhsgoncalves]]
* [[#2655]]: [fix] Urlencode when passing filenames to sqlite3 [[@uttarayan21]]
* [[#2684]]: Make PgListener recover from UnexpectedEof [[@hamiltop]]
* [[#2688]]: fix: Make rust_decimal and bigdecimal decoding more lenient [[@cameronbraid]]
* [[#2754]]: Is tests/x.py maintained? And I tried fix it. [[@qwerty2501]]
* [[#2784]]: fix: decode postgres time without subsecond [[@granddaifuku]]
* [[#2806]]: Depend on version of async-std with non-private spawn-blocking [[@A248]]
* [[#2820]]: fix: correct decoding of `rust_decimal::Decimal` for high-precision values [[@abonander]]
* [[#2822]]: issue #2821 Update error handling logic when opening a TCP connection [[@anupj]]
* [[#2826]]: chore: bump some sqlx-core dependencies [[@djc]]
* [[#2838]]: Fixes rust_decimal scale for Postgres [[@jkleinknox]]
* [[#2847]]: Fix comment in `sqlx migrate add` help text [[@cryeprecision]]
* [[#2850]]: fix(core): avoid unncessary wakeups in `try_stream!()` [[@abonander]]
* [[#2856]]: Prevent warnings running `cargo build` [[@nyurik]]
* [[#2864]]: fix(sqlite): use `AtomicUsize` for thread IDs [[@abonander]]
* [[#2892]]: Fixed force dropping bug [[@Vrajs16]]

[#2478]: https://github.com/launchbadge/sqlx/pull/2478
[#2545]: https://github.com/launchbadge/sqlx/pull/2545
[#2640]: https://github.com/launchbadge/sqlx/pull/2640
[#2655]: https://github.com/launchbadge/sqlx/pull/2655
[#2665]: https://github.com/launchbadge/sqlx/pull/2665
[#2684]: https://github.com/launchbadge/sqlx/pull/2684
[#2688]: https://github.com/launchbadge/sqlx/pull/2688
[#2701]: https://github.com/launchbadge/sqlx/pull/2701
[#2713]: https://github.com/launchbadge/sqlx/pull/2713
[#2752]: https://github.com/launchbadge/sqlx/pull/2752
[#2754]: https://github.com/launchbadge/sqlx/pull/2754
[#2769]: https://github.com/launchbadge/sqlx/pull/2769
[#2770]: https://github.com/launchbadge/sqlx/pull/2770
[#2782]: https://github.com/launchbadge/sqlx/pull/2782
[#2784]: https://github.com/launchbadge/sqlx/pull/2784
[#2793]: https://github.com/launchbadge/sqlx/pull/2793
[#2797]: https://github.com/launchbadge/sqlx/pull/2797
[#2801]: https://github.com/launchbadge/sqlx/pull/2801
[#2803]: https://github.com/launchbadge/sqlx/pull/2803
[#2806]: https://github.com/launchbadge/sqlx/pull/2806
[#2819]: https://github.com/launchbadge/sqlx/pull/2819
[#2820]: https://github.com/launchbadge/sqlx/pull/2820
[#2822]: https://github.com/launchbadge/sqlx/pull/2822
[#2826]: https://github.com/launchbadge/sqlx/pull/2826
[#2827]: https://github.com/launchbadge/sqlx/pull/2827
[#2832]: https://github.com/launchbadge/sqlx/pull/2832
[#2838]: https://github.com/launchbadge/sqlx/pull/2838
[#2847]: https://github.com/launchbadge/sqlx/pull/2847
[#2850]: https://github.com/launchbadge/sqlx/pull/2850
[#2856]: https://github.com/launchbadge/sqlx/pull/2856
[#2864]: https://github.com/launchbadge/sqlx/pull/2864
[#2865]: https://github.com/launchbadge/sqlx/pull/2865
[#2871]: https://github.com/launchbadge/sqlx/pull/2871
[#2873]: https://github.com/launchbadge/sqlx/pull/2873
[#2880]: https://github.com/launchbadge/sqlx/pull/2880
[#2882]: https://github.com/launchbadge/sqlx/pull/2882
[#2890]: https://github.com/launchbadge/sqlx/pull/2890
[#2892]: https://github.com/launchbadge/sqlx/pull/2892
[#2894]: https://github.com/launchbadge/sqlx/pull/2894

## 0.7.2 - 2023-09-25

23 pull requests were merged this release cycle.

### Added

* [[#2121]]: Add JSON support to `FromRow` derive [[@95ulisse]]
* [[#2533]]: Implement mysql_clear_password [[@ldanilek]]
* [[#2538]]: cli: add --target-version CLI flags for migrate run/revert [[@inahga]]
* [[#2577]]: supplement Postgres listen example with a small chat example [[@JockeM]]
* [[#2602]]: Support naming migrations sequentially [[@vmax]]
* [[#2634]]: Adding PgHasArrayType for &[u8;N] [[@snf]]
* [[#2646]]: Support for setting client certificate and key from bytes [[@wyhaya]]
* [[#2664]]: Automatically infer migration type [[@vmax]]
* [[#2712]]: Add impl for `Type`, `Decode`, and `Encode` for `Box<str>` and `Box<[u8]>` [[@grant0417]]

### Changed
* [[#2650]]: Cleanup format arguments [[@nyurik]]
* [[#2695]]: remove &mut PoolConnection from Executor docs [[@olback]]
    * This impl was removed in 0.7.0 because of coherence issues.
* [[#2706]]: Clarify where optional features should be enabled [[@kryptan]]
* [[#2717]]: Update README.md [[@fermanjj]]
* [[#2739]]: Bump mariadb CI images + mysql unpin [[@grooverdan]]
* [[#2742]]: Implemented poll_flush for Box<S:Socket> [[@bobozaur]]
* [[#2740]]: Remove sealed trait comments from documentation [[@bobozaur]]
* [[#2750]]: Fix #2384, bump flume to v0.11.0 [[@madadam]]
* [[#2757]]: Remove unused `remove_dir_all` crate from `sqlx-cli`, fixes RUSTSEC-2023-0018 [[@aldur]]

### Fixed

* [[#2624]]: Documentation typo: BYTE -> BINARY [[@sebastianv89]]
* [[#2628]]: docs: 0.7 is stable in the entire README [[@marcusirgens]]
* [[#2630]]: fix(postgres): fix buffer management in PgCopyIn::read_from [[@tsing]]
* [[#2651]]: Chore: Fix few build warnings, and make CI fail on warn [[@nyurik]]
* [[#2670]]: fix: ignore extra fields in Postgres describe parsing [[@abonander]]
* [[#2687]]: docs: Fix description of `min_connections` [[@hakoerber]]

[#2121]: https://github.com/launchbadge/sqlx/pull/2121
[#2533]: https://github.com/launchbadge/sqlx/pull/2533
[#2538]: https://github.com/launchbadge/sqlx/pull/2538
[#2577]: https://github.com/launchbadge/sqlx/pull/2577
[#2602]: https://github.com/launchbadge/sqlx/pull/2602
[#2624]: https://github.com/launchbadge/sqlx/pull/2624
[#2628]: https://github.com/launchbadge/sqlx/pull/2628
[#2630]: https://github.com/launchbadge/sqlx/pull/2630
[#2634]: https://github.com/launchbadge/sqlx/pull/2634
[#2646]: https://github.com/launchbadge/sqlx/pull/2646
[#2650]: https://github.com/launchbadge/sqlx/pull/2650
[#2651]: https://github.com/launchbadge/sqlx/pull/2651
[#2664]: https://github.com/launchbadge/sqlx/pull/2664
[#2670]: https://github.com/launchbadge/sqlx/pull/2670
[#2687]: https://github.com/launchbadge/sqlx/pull/2687
[#2695]: https://github.com/launchbadge/sqlx/pull/2695
[#2706]: https://github.com/launchbadge/sqlx/pull/2706
[#2712]: https://github.com/launchbadge/sqlx/pull/2712
[#2717]: https://github.com/launchbadge/sqlx/pull/2717
[#2739]: https://github.com/launchbadge/sqlx/pull/2739
[#2740]: https://github.com/launchbadge/sqlx/pull/2740
[#2742]: https://github.com/launchbadge/sqlx/pull/2742
[#2750]: https://github.com/launchbadge/sqlx/pull/2750
[#2757]: https://github.com/launchbadge/sqlx/pull/2757

## 0.7.1 - 2023-07-14

This release mainly addresses issues reported with the 0.7.0 release.

16 pull requests were merged this release cycle.

### Added
* [[#2551]]: Introduce build_query_scalar for QueryBuilder [[@iamquang95]]
* [[#2605]]: Implement Default for QueryBuilder [[@Xydez]]
* [[#2616]]: feat(sqlx-core): add table function to database error [[@saiintbrisson]]
* [[#2619]]: feat: allow opt-out of `PgHasArrayType` with `#[derive(sqlx::Type)]` [[@abonander]]
  * TL;DR: if you're getting errors from `#[derive(sqlx::Type)]` with `#[sqlx(transparent)]`
    regarding `PgHasArrayType` not being implemented, add `#[sqlx(no_pg_array)]` to fix.

### Changed
* [[#2566]]: improve docs about migration files [[@jnnnnn]]
* [[#2576]]: Major Version Update clap to 4.0 [[@titaniumtraveler]]
* [[#2597]]: Bump webpki-roots to v0.24 [[@paolobarbolini]]
* [[#2603]]: docs(changelog): be more verbose about offline mode breaking change [[@mrl5]]

### Fixed
* [[#2553]]: Implement `Clone` for `PoolOptions` manually (#2548) [[@alilleybrinker]]
* [[#2580]]: Update README.md now that 0.7.0 is no longer in alpha [[@saolof]]
* [[#2585]]: Fix for Issue #2549 - cannot use feature "rust_decimal" without also using "bigdecimal" [[@deneut]]
* [[#2586]]: Fix optional dependency on sqlx-macros [[@kitterion]]
* [[#2593]]: Correct mention of the `tls-native-tls` in the documentation. [[@denschub]]
* [[#2599]]: Remove incorrect CAST in test database cleanup for MySQL. [[@fd]]
* [[#2613]]: Fix readme.md to reduce confusion about optional features (decimal->rust_decimal) [[@vabka]]
* [[#2620]]: fix(sqlite/any): encode bool as integer [[@saiintbrisson]]

[#2551]: https://github.com/launchbadge/sqlx/pull/2551
[#2553]: https://github.com/launchbadge/sqlx/pull/2553
[#2566]: https://github.com/launchbadge/sqlx/pull/2566
[#2576]: https://github.com/launchbadge/sqlx/pull/2576
[#2580]: https://github.com/launchbadge/sqlx/pull/2580
[#2585]: https://github.com/launchbadge/sqlx/pull/2585
[#2586]: https://github.com/launchbadge/sqlx/pull/2586
[#2593]: https://github.com/launchbadge/sqlx/pull/2593
[#2597]: https://github.com/launchbadge/sqlx/pull/2597
[#2599]: https://github.com/launchbadge/sqlx/pull/2599
[#2603]: https://github.com/launchbadge/sqlx/pull/2603
[#2605]: https://github.com/launchbadge/sqlx/pull/2605
[#2613]: https://github.com/launchbadge/sqlx/pull/2613
[#2616]: https://github.com/launchbadge/sqlx/pull/2616
[#2619]: https://github.com/launchbadge/sqlx/pull/2619
[#2620]: https://github.com/launchbadge/sqlx/pull/2620

## 0.7.0 - 2023-06-30

At least **70 pull requests** were merged this release cycle! (The exact count is muddied with pull requests for alpha
releases and such.) And we gained 43 new contributors! Thank you to everyone who helped make this release a reality.

### Breaking
Many revisions were made to query analysis in the SQLite driver; these are all potentially breaking changes
as they can change the output of `sqlx::query!()` _et al_. We'd like to thank [[@tyrelr]] for their numerous PRs to
this area.

The MSSQL driver has been removed as it was not nearly at the same maturity level as the other drivers.
[As previously announced][sqlx-pro], we have plans to introduce a fully featured replacement as a premium offering,
alongside drivers for other proprietary databases, with the goal to support full-time development on SQLx. 

If interested, please email your inquiry to sqlx@launchbadge.com.

The offline mode for the queries has been changed to use a separate file per `query!()` invocation,
which is intended to reduce the number of conflicts when merging branches in a project that both modified queries.
This means that CLI flag `--merged` is no longer supported. See [[#2363]] for details and make sure that your
`sqlx-cli` version is in sync with the `sqlx` version in your project.

The type ascription override syntax for the query macros has been deprecated,
as parse support for it has been removed in `syn 2.0`, which we'll be upgrading to in the next breaking release. 
This can be replaced with type overrides using casting syntax (`as`). 
See [[#2483]] for details.

* [[#1946]]: Fix compile time verification performance regression for sqlite [[@liningpan]]
* [[#1960]]: Fix sqlite update return and order by type inference [[@tyrelr]]
* [[#1984]]: Sqlite EXPLAIN type inference improvements [[@rongcuid]]
* [[#2039]]: Break drivers out into separate crates, clean up some technical debt [[@abonander]]
    * All deprecated items have been removed.
    * The `mssql` feature and associated database driver has been deleted from the source tree. It will return as part of our planned SQLx Pro offering as a from-scratch rewrite with extra features (such as TLS) and type integrations that were previously missing.
    * The `runtime-actix-*` features have been deleted. They were previously changed to be aliases of their `runtime-tokio-*` counterparts for backwards compatibility reasons, but their continued existence is misleading as SQLx has no special knowledge of Actix anymore.
        * To fix, simply replace the `runtime-actix-*` feature with its `runtime-tokio-*` equivalent.
    * The `git2` feature has been removed. This was a requested integration from a while ago that over time made less and less sense to be part of SQLx itself. We have to be careful with the crates we add to our public API as each one introduces yet another semver hazard. The expected replacement is to make `#[derive(sqlx::Type)]` useful enough that users can write wrapper types for whatever they want to use without SQLx needing to be specifically aware of it.
    * The `Executor` impls for `Transaction` and `PoolConnection` have been deleted because they cannot exist in the new crate architecture without rewriting the `Executor` trait entirely.
        * To fix this breakage, simply add a dereference where an `impl Executor` is expected, as they both dereference to the inner connection type which will still implement it:
            * `&mut transaction` -> `&mut *transaction`
            * `&mut connection` -> `&mut *connection`
        * These cannot be blanket impls as it triggers an overflow in the compiler due to the lack of lazy normalization, and
          the driver crates cannot provide their own impls due to the orphan rule.
        * We're expecting to do another major refactor of traits to incorporate generic associated types (GAT).
          This will mean another major release of SQLx but ideally most API usage will not need to change significantly, if at all.
    * The fields of `Migrator` are now `#[doc(hidden)]` and semver-exempt; they weren't meant to be public.
    * The `offline` feature has been removed from the `sqlx` facade crate and is enabled unconditionally as most users are expected to have enabled it anyway and disabling it doesn't seem to appreciably affect compile times.
    * The `decimal` feature has been renamed to `rust_decimal` to match the crate it actually provides integrations for.
    * `AnyDriver` and `AnyConnection` now require either `sqlx::any::install_drivers()` or `sqlx::any::install_default_drivers()` to be called at some point during the process' lifetime before the first connection is made, as the set of possible drivers is now determined at runtime. This was determined to be the least painful way to provide knowledge of database drivers to `Any` without them being hardcoded.
    * The `AnyEncode` trait has been removed.
* [[#2109]]: feat: better database errors [[@saiintbrisson]]
* [[#2094]]: Update libsqlite3-sys to 0.25.1 [[@penberg]]
  * Alongside this upgrade, we are now considering the linkage to `libsqlite3-sys` to be **semver-exempt**,
    and we reserve the right to upgrade it as necessary. If you are using `libsqlite3-sys` directly or a crate that
    links it such as `rusqlite`, you should pin the versions of both crates to avoid breakages from `cargo update`:
```toml
[dependencies]
sqlx = { version = "=0.7.0", features = ["sqlite"] }
rusqlite = "=0.29.0"
```
* [[#2132]]: fix: use owned Builder pattern for ConnectOptions [[@ar3s3ru]]
* [[#2253]]: Sqlite describe fixes [[@tyrelr]]
* [[#2285]]: `time`: Assume UTC when decoding a DATETIME column in sqlite [[@nstinus]]
* [[#2363]]: [offline] Change prepare to one-file-per-query [[@cycraig]]
* [[#2387]]: PATCH: bump libsqlite3-sys to patched version [[@grantkee]]
* [[#2409]]: fix(#2407): respect the HaltIfNull opcode when determining nullability [[@arlyon]]
* [[#2459]]: limit the number of instructions that can be evaluated [[@tyrelr]]
* [[#2467]]: Add and improve sqlite describe performance benchmarks [[@tyrelr]]
* [[#2491]]: sqlite date macro support [[@Arcayr]]
    * Changes `OffsetDateTime` to be the first type used when deserializing a `timestamp` type.
* [[#2496]]: Bump to libsqlite3-sys 0.26 [[@mdecimus]]
* [[#2508]]: Sqlite analytical [[@tyrelr]]


### Added
* [[#1850]]: Add client SSL authentication using key-file for Postgres, MySQL and MariaDB [[@ThibsG]]
* [[#2088]]: feat: Add set_connect_options method to Pool [[@moatra]]
* [[#2113]]: Expose PoolOptions for reading [[@FSMaxB]]
* [[#2115]]: Allow using complex types in `try_from` when deriving `FromRow` [[@95ulisse]]
* [[#2116]]: [SQLite] Add option to execute `PRAGMA optimize;` on close of a connection [[@miles170]]
* [[#2189]]: Added regexp support in sqlite [[@VictorKoenders]]
* [[#2224]]: Add From impls for Json [[@dbeckwith]]
* [[#2256]]: add progress handler support to sqlite [[@nbaztec]]
* [[#2366]]: Allow ignoring attributes for deriving FromRow [[@grgi]]
* [[#2369]]: new type support in query_as [[@0xdeafbeef]]
* [[#2379]]: feat: add `Connection::shrink_buffers`, `PoolConnection::close` [[@abonander]]
* [[#2400]]: fix(docs): example of `sqlx_macros_unstable` in config.toml [[@df51d]]
* [[#2469]]: Add Simple format for Uuid for MySQL & SQLite. [[@MidasLamb]]
* [[#2483]]: chore: add deprecation notice for type ascription use [[@saiintbrisson]]
* [[#2506]]: add args to query builder (#2494) [[@cemoktra]]
* [[#2554]]: Impl `AsMut` for advisory lock types (#2520) [[@alilleybrinker]]
* [[#2559]]: Add CLI autocompletion using clap_complete [[@titaniumtraveler]]


### Changed
* [[#2185]]: Initial work to switch to `tracing` [[@CosmicHorrorDev]]
* [[#2193]]: Start testing on Postgres 15 and drop Postgres 10 [[@paolobarbolini]]
    * We reserve the right to drop support for end-of-lifed database versions [as discussed in our FAQ][faq-db-version].
* [[#2213]]: Use `let else` statements in favor of macro [[@OverHash]]
* [[#2365]]: Update dependencies [[@paolobarbolini]]
* [[#2371]]: Disable rustls crate logging feature by default up to date [[@sergeiivankov]]
* [[#2373]]: chore: Use tracing's fields to get structured logs [[@jaysonsantos]]
* [[#2393]]: Lower default logging level for statements to Debug [[@bnoctis]]
* [[#2445]]: Traverse symlinks when resolving migrations [[@tgeoghegan]]
* [[#2485]]: chore(sqlx-postgres): replace `dirs` with `home` & `etcetera` [[@utkarshgupta137]]
* [[#2515]]: Bump mac_address to 1.1.5 [[@repnop]]
* [[#2440]]: Update rustls to 0.21, webpki-roots to 0.23 [[@SergioBenitez]]
* [[#2563]]: Update rsa to 0.9 [[@paolobarbolini]]
* [[#2564]]: Update bitflags to v2 [[@paolobarbolini]]
* [[#2565]]: Bump indexmap and ahash [[@paolobarbolini]]
* [[#2574]]: doc: make it clear that `ConnectOptions` types impl `FromStr` [[@abonander]]

### Fixed
* [[#2098]]: Fix sqlite compilation [[@cycraig]]
* [[#2120]]: fix logical merge conflict [[@tyrelr]]
* [[#2133]]: Postgres OID resolution query does not take into account current `search_path` [[@95ulisse]]
* [[#2156]]: Fixed typo. [[@cdbfoster]]
* [[#2179]]: fix: ensures recover from fail with PgCopyIn [[@andyquinterom]]
* [[#2200]]: Run CI on *-dev branch [[@joehillen]]
* [[#2222]]: Add context to confusing sqlx prepare parse error [[@laundmo]]
* [[#2271]]: feat: support calling Postgres procedures with the macros [[@bgeron]]
* [[#2282]]: Don't run EXPLAIN nullability analysis on Materialize [[@benesch]]
* [[#2319]]: Set whoami default-features to false [[@thedodd]]
* [[#2352]]: Preparing 0.7.0-alpha.1 release [[@abonander]]
* [[#2355]]: Fixed the example code for `sqlx::test` [[@kenkoooo]]
* [[#2367]]: Fix sqlx-cli create, drop, migrate [[@cycraig]]
* [[#2376]]: fix(pool): close when last handle is dropped, extra check in `try_acquire` [[@abonander]]
* [[#2378]]: Fix README build badge [[@dbrgn]]
* [[#2398]]: fix(prepare): store temporary query files inside the workspace [[@aschey]]
* [[#2402]]: fix: drop old time 0.1.44 dep [[@codahale]]
* [[#2413]]: fix(macros-core): use of undeclared `tracked_path` [[@df51d]]
* [[#2420]]: Enable runtime-tokio feature of sqlx when building sqlx-cli [[@paolobarbolini]]
* [[#2453]]: in README.md, correct spelling and grammar [[@vizvasrj]]
* [[#2454]]: fix: ensure fresh test db's aren't accidentally deleted by do_cleanup [[@phlip9]]
* [[#2507]]: Exposing the Oid of PostgreSQL types [[@Razican]]
* [[#2519]]: Use ::std::result::Result::Ok in output.rs [[@southball]]
* [[#2569]]: Fix broken links to mysql error documentation [[@titaniumtraveler]]
* [[#2570]]: Add a newline to the generated JSON files [[@nyurik]]
* [[#2572]]: Do not panic when `PrepareOk` fails to decode [[@stepantubanov]]
* [[#2573]]: fix(sqlite) Do not drop notify mutex guard until after condvar is triggered [[@andrewwhitehead]]

[sqlx-pro]: https://github.com/launchbadge/sqlx/discussions/1616

[faq-db-version]: https://github.com/launchbadge/sqlx/blob/main/FAQ.md#what-database-versions-does-sqlx-support

[#1850]: https://github.com/launchbadge/sqlx/pull/1850
[#1946]: https://github.com/launchbadge/sqlx/pull/1946
[#1960]: https://github.com/launchbadge/sqlx/pull/1960
[#1984]: https://github.com/launchbadge/sqlx/pull/1984
[#2039]: https://github.com/launchbadge/sqlx/pull/2039
[#2088]: https://github.com/launchbadge/sqlx/pull/2088
[#2092]: https://github.com/launchbadge/sqlx/pull/2092
[#2094]: https://github.com/launchbadge/sqlx/pull/2094
[#2098]: https://github.com/launchbadge/sqlx/pull/2098
[#2109]: https://github.com/launchbadge/sqlx/pull/2109
[#2113]: https://github.com/launchbadge/sqlx/pull/2113
[#2115]: https://github.com/launchbadge/sqlx/pull/2115
[#2116]: https://github.com/launchbadge/sqlx/pull/2116
[#2120]: https://github.com/launchbadge/sqlx/pull/2120
[#2132]: https://github.com/launchbadge/sqlx/pull/2132
[#2133]: https://github.com/launchbadge/sqlx/pull/2133
[#2156]: https://github.com/launchbadge/sqlx/pull/2156
[#2179]: https://github.com/launchbadge/sqlx/pull/2179
[#2185]: https://github.com/launchbadge/sqlx/pull/2185
[#2189]: https://github.com/launchbadge/sqlx/pull/2189
[#2193]: https://github.com/launchbadge/sqlx/pull/2193
[#2200]: https://github.com/launchbadge/sqlx/pull/2200
[#2213]: https://github.com/launchbadge/sqlx/pull/2213
[#2222]: https://github.com/launchbadge/sqlx/pull/2222
[#2224]: https://github.com/launchbadge/sqlx/pull/2224
[#2253]: https://github.com/launchbadge/sqlx/pull/2253
[#2256]: https://github.com/launchbadge/sqlx/pull/2256
[#2271]: https://github.com/launchbadge/sqlx/pull/2271
[#2282]: https://github.com/launchbadge/sqlx/pull/2282
[#2285]: https://github.com/launchbadge/sqlx/pull/2285
[#2319]: https://github.com/launchbadge/sqlx/pull/2319
[#2352]: https://github.com/launchbadge/sqlx/pull/2352
[#2355]: https://github.com/launchbadge/sqlx/pull/2355
[#2363]: https://github.com/launchbadge/sqlx/pull/2363
[#2365]: https://github.com/launchbadge/sqlx/pull/2365
[#2366]: https://github.com/launchbadge/sqlx/pull/2366
[#2367]: https://github.com/launchbadge/sqlx/pull/2367
[#2369]: https://github.com/launchbadge/sqlx/pull/2369
[#2371]: https://github.com/launchbadge/sqlx/pull/2371
[#2373]: https://github.com/launchbadge/sqlx/pull/2373
[#2376]: https://github.com/launchbadge/sqlx/pull/2376
[#2378]: https://github.com/launchbadge/sqlx/pull/2378
[#2379]: https://github.com/launchbadge/sqlx/pull/2379
[#2387]: https://github.com/launchbadge/sqlx/pull/2387
[#2393]: https://github.com/launchbadge/sqlx/pull/2393
[#2398]: https://github.com/launchbadge/sqlx/pull/2398
[#2400]: https://github.com/launchbadge/sqlx/pull/2400
[#2402]: https://github.com/launchbadge/sqlx/pull/2402
[#2408]: https://github.com/launchbadge/sqlx/pull/2408
[#2409]: https://github.com/launchbadge/sqlx/pull/2409
[#2413]: https://github.com/launchbadge/sqlx/pull/2413
[#2420]: https://github.com/launchbadge/sqlx/pull/2420
[#2440]: https://github.com/launchbadge/sqlx/pull/2440
[#2445]: https://github.com/launchbadge/sqlx/pull/2445
[#2453]: https://github.com/launchbadge/sqlx/pull/2453
[#2454]: https://github.com/launchbadge/sqlx/pull/2454
[#2459]: https://github.com/launchbadge/sqlx/pull/2459
[#2467]: https://github.com/launchbadge/sqlx/pull/2467
[#2469]: https://github.com/launchbadge/sqlx/pull/2469
[#2483]: https://github.com/launchbadge/sqlx/pull/2483
[#2485]: https://github.com/launchbadge/sqlx/pull/2485
[#2491]: https://github.com/launchbadge/sqlx/pull/2491
[#2496]: https://github.com/launchbadge/sqlx/pull/2496
[#2506]: https://github.com/launchbadge/sqlx/pull/2506
[#2507]: https://github.com/launchbadge/sqlx/pull/2507
[#2508]: https://github.com/launchbadge/sqlx/pull/2508
[#2515]: https://github.com/launchbadge/sqlx/pull/2515
[#2519]: https://github.com/launchbadge/sqlx/pull/2519
[#2554]: https://github.com/launchbadge/sqlx/pull/2554
[#2559]: https://github.com/launchbadge/sqlx/pull/2559
[#2563]: https://github.com/launchbadge/sqlx/pull/2563
[#2564]: https://github.com/launchbadge/sqlx/pull/2564
[#2565]: https://github.com/launchbadge/sqlx/pull/2565
[#2569]: https://github.com/launchbadge/sqlx/pull/2569
[#2570]: https://github.com/launchbadge/sqlx/pull/2570
[#2572]: https://github.com/launchbadge/sqlx/pull/2572
[#2573]: https://github.com/launchbadge/sqlx/pull/2573
[#2574]: https://github.com/launchbadge/sqlx/pull/2574

## 0.6.3 - 2023-03-21

This is a hotfix to address the breakage caused by transitive dependencies upgrading to `syn = "2"`.

We set `default-features = false` for our dependency on `syn = "1"` to be good crates.io citizens, 
but failed to enable the features we actually used, which went undetected because we transitively depended on
`syn` with the default features enabled through other crates, 
and so they were also on for us because features are additive.

When those other dependencies upgraded to `syn = "2"` it was no longer enabling those features for us, 
and so compilation broke for projects that don't also depend on `syn = "1"`, transitively or otherwise.

There is no PR for this fix as there was no longer a dedicated development branch for `0.6`, 
but discussion can be found in [issue #2418].

As of this release, the `0.7` release is in alpha and so development is no longer occurring against `0.6`.
This fix will be forward-ported to `0.7`.

[issue #2418]: https://github.com/launchbadge/sqlx/issues/2418

## 0.6.2 - 2022-09-14

[25 pull requests][0.6.2-prs] were merged this release cycle.

### Added
* [[#1081]]: Add `try_from` attribute for `FromRow` derive [[@zzhengzhuo]]
    * Exemplifies "out of sight, out of mind." It's surprisingly easy to forget about PRs when they get pushed onto
      the second page. We'll be sure to clean out the backlog for 0.7.0.
* [[#2014]]: Support additional SQLCipher options in SQLite driver. [[@szymek156]]
* [[#2052]]: Add issue templates [[@abonander]]
* [[#2053]]: Add documentation for `IpAddr` support in Postgres [[@rakshith-ravi]]
* [[#2062]]: Add extension support for SQLite [[@bradfier]]
* [[#2063]]: customizable db locking during migration [[@fuzzbuck]]

### Changed
* [[#2025]]: Bump sqlformat to 2.0 [[@NSMustache]]
* [[#2056]]: chore: Switch to sha1 crate [[@stoically]]
* [[#2071]]: Use cargo check consistently in `prepare` [[@cycraig]]

### Fixed
* [[#1991]]: Ensure migration progress is not lost for Postgres, MySQL and SQLite. [[@crepererum]]
* [[#2023]]: Fix expansion of `#[sqlx(flatten)]` for `FromRow` derive [[@RustyYato]]
* [[#2028]]: Use fully qualified path when forwarding to `#[test]` from `#[sqlx::test]` [[@alexander-jackson]]
* [[#2040]]: Fix typo in `FromRow` docs [[@zlidner]]
* [[#2046]]: added flag for PIPES_AS_CONCAT connection setting for MySQL to fix #2034 [[@marcustut]]
* [[#2055]]: Use unlock notify also on `sqlite3_exec`  [[@madadam]]
* [[#2057]]: Make begin,commit,rollback cancel-safe in sqlite  [[@madadam]]
* [[#2058]]: fix typo in documentation [[@lovasoa]]
* [[#2067]]: fix(docs): close code block in query_builder.rs [[@abonander]]
* [[#2069]]: Fix `prepare` race condition in workspaces [[@cycraig]]\
    * NOTE: this changes the directory structure under `target/` that `cargo sqlx prepare` depends on.
      If you use offline mode in your workflow, please rerun `cargo install sqlx-cli` to upgrade.
* [[#2072]]: SqliteConnectOptions typo [[@fasterthanlime]]
* [[#2074]]: fix: mssql uses unsigned for tinyint instead of signed [[@he4d]]
* [[#2081]]: close unnamed portal after each executed extended query [[@DXist]]
* [[#2086]]: PgHasArrayType for transparent types fix. [[@Wopple]]
    * NOTE: this is a breaking change and has been postponed to 0.7.0.
* [[#2089]]: fix: Remove default chrono dep on time for sqlx-cli [[@TravisWhitehead]]
* [[#2091]]: Sqlite explain plan log efficiency [[@tyrelr]]

[0.6.2-prs]: https://github.com/launchbadge/sqlx/pulls?q=is%3Apr+is%3Aclosed+merged%3A2022-08-04..2022-09-14+

[#1081]: https://github.com/launchbadge/sqlx/pull/1081
[#1991]: https://github.com/launchbadge/sqlx/pull/1991
[#2014]: https://github.com/launchbadge/sqlx/pull/2014
[#2023]: https://github.com/launchbadge/sqlx/pull/2023
[#2025]: https://github.com/launchbadge/sqlx/pull/2025
[#2028]: https://github.com/launchbadge/sqlx/pull/2028
[#2040]: https://github.com/launchbadge/sqlx/pull/2040
[#2046]: https://github.com/launchbadge/sqlx/pull/2046
[#2052]: https://github.com/launchbadge/sqlx/pull/2052
[#2053]: https://github.com/launchbadge/sqlx/pull/2053
[#2055]: https://github.com/launchbadge/sqlx/pull/2055
[#2056]: https://github.com/launchbadge/sqlx/pull/2056
[#2057]: https://github.com/launchbadge/sqlx/pull/2057
[#2058]: https://github.com/launchbadge/sqlx/pull/2058
[#2062]: https://github.com/launchbadge/sqlx/pull/2062
[#2063]: https://github.com/launchbadge/sqlx/pull/2063
[#2067]: https://github.com/launchbadge/sqlx/pull/2067
[#2069]: https://github.com/launchbadge/sqlx/pull/2069
[#2071]: https://github.com/launchbadge/sqlx/pull/2071
[#2072]: https://github.com/launchbadge/sqlx/pull/2072
[#2074]: https://github.com/launchbadge/sqlx/pull/2074
[#2081]: https://github.com/launchbadge/sqlx/pull/2081
[#2086]: https://github.com/launchbadge/sqlx/pull/2086
[#2089]: https://github.com/launchbadge/sqlx/pull/2089
[#2091]: https://github.com/launchbadge/sqlx/pull/2091

## 0.6.1 - 2022-08-02

[33 pull requests][0.6.1-prs] were merged this release cycle.

### Added
* [[#1495]]: Add example for manual implementation of the `FromRow` trait [[@Erik1000]]
* [[#1822]]: (Postgres) Add support for `std::net::IpAddr` [[@meh]]
    * Decoding returns an error if the `INET` value in Postgres is a prefix and not a full address
      (`/32` for IPv4, `/128` for IPv6).
* [[#1865]]: Add SQLite support for the `time` crate [[@johnbcodes]]
* [[#1902]]: Add an example of how to use `QueryBuilder::separated()` [[@sbeckeriv]]
* [[#1917]]: Added docs for `sqlx::types::Json` [[@jayy-lmao]]
* [[#1919]]: Implement `Clone` for `PoolOptions` [[@Thomasdezeeuw]]
* [[#1953]]: Support Rust arrays in Postgres [[@e00E]]
* [[#1954]]: Add `push_tuples` for `QueryBuilder` [[@0xdeafbeef]]
* [[#1959]]: Support `#[sqlx(flatten)]` attribute in `FromRow` [[@TheoOiry]]
* [[#1967]]: Add example with external query files [[@JoeyMckenzie]]
* [[#1985]]: Add `query_builder::Separated::push_bind_unseparated()` [[@0xdeafbeef]]
* [[#2001]]: Implement `#[sqlx::test]` for general use
    * Includes automatic database management, migration and fixture application.
    * Drops support for end-of-lifed database versions, see PR for details.
* [[#2005]]: `QueryBuilder` improvements [[@abonander]]
    * Raw SQL getters, new method to build `QueryAs` instead of `Query`.
* [[#2013]]: (SQLite) Allow VFS to be set as URL query parameter [[@liningpan]] 

### Changed
* [[#1679]]: refactor: alias actix-* features to their equivalent tokio-* features [[@robjtede]]
* [[#1906]]: replaced all uses of "uri" to "url" [[@RomainStorai]]
* [[#1965]]: SQLite improvements [[@abonander]]
* [[#1977]]: Docs: clarify relationship between `query_as!()` and `FromRow` [[@abonander]]
* [[#2003]]: Replace `dotenv` with `dotenvy` [[@abonander]]

### Fixed
* [[#1802]]: Try avoiding a full clean in `cargo sqlx prepare --merged` [[@LovecraftianHorror]]
* [[#1848]]: Fix type info access in `Any` database driver [[@raviqqe]]
* [[#1910]]: Set `CARGO_TARGET_DIR` when compiling queries [[@sedrik]]
* [[#1915]]: Pool: fix panic when using callbacks [[@abonander]]
* [[#1930]]: Don't cache SQLite connection for macros [[@LovecraftianHorror]]
* [[#1948]]: Fix panic in Postgres `BYTEA` decode [[@e00E]]
* [[#1955]]: Fix typo in FAQ [[@kenkoooo]]
* [[#1968]]: (Postgres) don't panic if `S` or `V` notice fields are not UTF-8 [[@abonander]]
* [[#1969]]: Fix sqlx-cli build [[@ivan]]
* [[#1974]]: Use the `rust-cache` action for CI [[@abonander]]
* [[#1988]]: Agree on a single default runtime for the whole workspace [[@crepererum]]
* [[#1989]]: Fix panics in `PgListener` [[@crepererum]]
* [[#1990]]: Switch `master` to `main` in docs [[@crepererum]]
    * The change had already been made in the repo, the docs were out of date.
* [[#1993]]: Update versions in quickstart examples in README [[@UramnOIL]]

[0.6.1-prs]: https://github.com/launchbadge/sqlx/pulls?page=1&q=is%3Apr+is%3Aclosed+merged%3A2022-06-17..2022-08-02

[#1906]: https://github.com/launchbadge/sqlx/pull/1906
[#1495]: https://github.com/launchbadge/sqlx/pull/1495
[#1679]: https://github.com/launchbadge/sqlx/pull/1679
[#1802]: https://github.com/launchbadge/sqlx/pull/1802
[#1822]: https://github.com/launchbadge/sqlx/pull/1822
[#1848]: https://github.com/launchbadge/sqlx/pull/1848
[#1865]: https://github.com/launchbadge/sqlx/pull/1865
[#1902]: https://github.com/launchbadge/sqlx/pull/1902
[#1910]: https://github.com/launchbadge/sqlx/pull/1910
[#1915]: https://github.com/launchbadge/sqlx/pull/1915
[#1917]: https://github.com/launchbadge/sqlx/pull/1917
[#1919]: https://github.com/launchbadge/sqlx/pull/1919
[#1930]: https://github.com/launchbadge/sqlx/pull/1930
[#1948]: https://github.com/launchbadge/sqlx/pull/1948
[#1953]: https://github.com/launchbadge/sqlx/pull/1953
[#1954]: https://github.com/launchbadge/sqlx/pull/1954
[#1955]: https://github.com/launchbadge/sqlx/pull/1955
[#1959]: https://github.com/launchbadge/sqlx/pull/1959
[#1965]: https://github.com/launchbadge/sqlx/pull/1965
[#1967]: https://github.com/launchbadge/sqlx/pull/1967
[#1968]: https://github.com/launchbadge/sqlx/pull/1968
[#1969]: https://github.com/launchbadge/sqlx/pull/1969
[#1974]: https://github.com/launchbadge/sqlx/pull/1974
[#1977]: https://github.com/launchbadge/sqlx/pull/1977
[#1985]: https://github.com/launchbadge/sqlx/pull/1985
[#1988]: https://github.com/launchbadge/sqlx/pull/1988
[#1989]: https://github.com/launchbadge/sqlx/pull/1989
[#1990]: https://github.com/launchbadge/sqlx/pull/1990
[#1993]: https://github.com/launchbadge/sqlx/pull/1993
[#2001]: https://github.com/launchbadge/sqlx/pull/2001
[#2003]: https://github.com/launchbadge/sqlx/pull/2003
[#2005]: https://github.com/launchbadge/sqlx/pull/2005
[#2013]: https://github.com/launchbadge/sqlx/pull/2013

## 0.6.0 - 2022-06-16

This release marks the end of the 0.5.x series of releases and contains a number of breaking changes,
mainly to do with backwards-incompatible dependency upgrades. 

As we foresee many more of these in the future, we [surveyed the community] on how to handle this;
the consensus appears to be "just release breaking changes more often." 

As such, we expect the 0.6.x release series to be a shorter one.

[39 pull requests(!)][0.6.0-prs] (not counting "prepare 0.5.12 release", of course) were merged this release cycle.

### Breaking
* [[#1384]]: (Postgres) Move `server_version_num` from trait to inherent impl [[@AtkinsChang]]
* [[#1426]]: Bump `ipnetwork` to 0.19 [[@paolobarbolini]]
* [[#1455]]: Upgrade `time` to 0.3 [[@paolobarbolini]]
* [[#1505]]: Upgrade `rustls` to 0.20 [[@paolobarbolini]]
    * Fortunately, future upgrades should not be breaking as `webpki` is no longer exposed in the API.
* [[#1529]]: Upgrade `bigdecimal` to 0.3 [[@e00E]]
* [[#1602]]: postgres: use `Oid` everywhere instead of `u32` [[@paolobarbolini]]
    * This drops the `Type`, `Decode`, `Encode` impls for `u32` for Postgres as it was misleading.
      Postgres doesn't support unsigned ints without using an extension. These impls were decoding Postgres `OID`s
      as bare `u32`s without any context (and trying to bind a `u32` to a query would produce an `OID` value in SQL).
      This changes that to use a newtype instead, for clarity.
* [[#1612]]: Make all `ConnectOptions` types cloneable [[@05storm26]]
* [[#1618]]: SQLite `chrono::DateTime<FixedOffset>` timezone fix [[@05storm26]]
    * `DateTime<FixedOffset>` will be stored in SQLite with the correct timezone instead of always in UTC.
      This was flagged as a "potentially breaking change" since it changes how dates are sent to SQLite.
* [[#1733]]: Update `git2` to 0.14 [[@joshtriplett]]
* [[#1734]]: Make `PgLTree::push()` infallible and take `PgLTreeLabel` directly [[@sebpuetz]]
* [[#1785]]: Fix Rust type for SQLite `REAL` [[@pruthvikar]]
    * Makes the macros always map a `REAL` column to `f64` instead of `f32` as SQLite uses **only** 64-bit floats.
* [[#1816]]: Improve SQLite support for sub-queries and CTEs [[@tyrelr]]
    * This likely will change the generated code for some invocations `sqlx::query!()` with SQLite.
* [[#1821]]: Update `uuid` crate to v1 [[@paolobarbolini]]
* [[#1901]]: Pool fixes and breaking changes [[@abonander]]
    * Renamed `PoolOptions::connect_timeout` to `acquire_timeout` for clarity.
    * Changed the expected signatures for `PoolOptions::after_connect`, `before_acquire`, `after_release`
    * Changed the signature for `Pool::close()` slightly
        * Now eagerly starts the pool closing, `.await`ing is only necessary if you want to ensure a graceful shutdown.
    * Deleted `PoolConnection::release()` which was previously deprecated in favor of `PoolConnection::detach()`.
    * Fixed connections getting leaked even when calling `.close()`.
* [[#1748]]: Derive `PgHasArrayType` for `#[sqlx(transparent)]` types [[@carols10cents]]
    * This change was released with 0.5.12 but [we didn't realize it was a breaking change] at the time.  
      It was reverted in 0.5.13 and postponed until this release.

### Added
* [[#1843]]: Expose some useful methods on `PgValueRef` [[@mfreeborn]]
* [[#1889]]: SQLx-CLI: add `--connect-timeout` [[@abonander]]
    * Adds a default 10 second connection timeout to all commands.
* [[#1890]]: Added test for mssql LoginAck [[@walf443]]
* [[#1891]]: Added test for mssql ProtocolInfo [[@walf443]]
* [[#1892]]: Added test for mssql ReturnValue [[@walf443]]
* [[#1895]]: Add support for `i16` to `Any` driver [[@EthanYuan]]
* [[#1897]]: Expose `ConnectOptions` and `PoolOptions` on `Pool` and database name on `PgConnectOptions` [[@Nukesor]]

### Changed
* [[#1782]]: Reuse a cached DB connection instead of always opening a new one for `sqlx-macros` [[@LovecraftianHorror]]
* [[#1807]]: Bump remaining dependencies [[@paolobarbolini]]
* [[#1808]]: Update to edition 2021 [[@paolobarbolini]]
    * Note that while SQLx [does not officially track an MSRV] and only officially supports the latest stable Rust, 
      this effectively places a lower bound of 1.56.0 on the range of versions it may work with.
* [[#1823]]: (sqlx-macros) Ignore deps when getting metadata for workspace root [[@LovecraftianHorror]]
* [[#1831]]: Update `crc` to 3.0 [[@djc]]
* [[#1887]]: query_as: don't stop stream after decoding error [[@lovasoa]]

### Fixed
* [[#1814]]: SQLx-cli README: move `Usage` to the same level as `Install` [[@tobymurray]]
* [[#1815]]: SQLx-cli README: reword "building in offline mode" [[@tobymurray]]
* [[#1818]]: Trim `[]` from host string before passing to TcpStream [[@smonv]]
    * This fixes handling of database URLs with IPv6 hosts.
* [[#1842]]: Fix usage of `serde_json` in macros [[@mfreeborn]]
* [[#1855]]: Postgres: fix panics on unknown type OID when decoding [[@demurgos]] 
* [[#1856]]: MySQL: support COLLATE_UTF8MB4_0900_AI_CI [[@scottwey]]
    * Fixes the MySQL driver thinking text columns are bytestring columns when querying against a Planetscale DB.
* [[#1861]]: MySQL: avoid panic when streaming packets are empty [[@e-rhodes]]
* [[#1863]]: Fix nullability check for inner joins in Postgres [[@OskarPersson]]
* [[#1881]]: Fix `field is never read` warnings on Postgres test [[@walf443]]
* [[#1882]]: Fix `unused result must be used` warnings [[@walf443]]
* [[#1888]]: Fix migration checksum comparison during `sqlx migrate info` [[@mdtusz]]
* [[#1894]]: Fix typos [[@kianmeng]]

[surveyed the community]: https://github.com/launchbadge/sqlx/issues/1796
[0.6.0-prs]: https://github.com/launchbadge/sqlx/pulls?page=2&q=is%3Apr+is%3Amerged+merged%3A2022-04-14..2022-06-16
[does not officially track an MSRV]: /FAQ.md#what-versions-of-rust-does-sqlx-support-what-is-sqlxs-msrv
[we didn't realize it was a breaking change]: https://github.com/launchbadge/sqlx/pull/1800#issuecomment-1099898932

[#1384]: https://github.com/launchbadge/sqlx/pull/1384
[#1426]: https://github.com/launchbadge/sqlx/pull/1426
[#1455]: https://github.com/launchbadge/sqlx/pull/1455
[#1505]: https://github.com/launchbadge/sqlx/pull/1505
[#1529]: https://github.com/launchbadge/sqlx/pull/1529
[#1602]: https://github.com/launchbadge/sqlx/pull/1602
[#1612]: https://github.com/launchbadge/sqlx/pull/1612
[#1618]: https://github.com/launchbadge/sqlx/pull/1618
[#1733]: https://github.com/launchbadge/sqlx/pull/1733
[#1734]: https://github.com/launchbadge/sqlx/pull/1734
[#1782]: https://github.com/launchbadge/sqlx/pull/1782
[#1785]: https://github.com/launchbadge/sqlx/pull/1785
[#1807]: https://github.com/launchbadge/sqlx/pull/1807
[#1808]: https://github.com/launchbadge/sqlx/pull/1808
[#1814]: https://github.com/launchbadge/sqlx/pull/1814
[#1815]: https://github.com/launchbadge/sqlx/pull/1815
[#1816]: https://github.com/launchbadge/sqlx/pull/1816
[#1818]: https://github.com/launchbadge/sqlx/pull/1818
[#1821]: https://github.com/launchbadge/sqlx/pull/1821
[#1823]: https://github.com/launchbadge/sqlx/pull/1823
[#1831]: https://github.com/launchbadge/sqlx/pull/1831
[#1842]: https://github.com/launchbadge/sqlx/pull/1842
[#1843]: https://github.com/launchbadge/sqlx/pull/1843
[#1855]: https://github.com/launchbadge/sqlx/pull/1855
[#1856]: https://github.com/launchbadge/sqlx/pull/1856
[#1861]: https://github.com/launchbadge/sqlx/pull/1861
[#1863]: https://github.com/launchbadge/sqlx/pull/1863
[#1881]: https://github.com/launchbadge/sqlx/pull/1881
[#1882]: https://github.com/launchbadge/sqlx/pull/1882
[#1887]: https://github.com/launchbadge/sqlx/pull/1887
[#1888]: https://github.com/launchbadge/sqlx/pull/1888
[#1889]: https://github.com/launchbadge/sqlx/pull/1889
[#1890]: https://github.com/launchbadge/sqlx/pull/1890
[#1891]: https://github.com/launchbadge/sqlx/pull/1891
[#1892]: https://github.com/launchbadge/sqlx/pull/1892
[#1894]: https://github.com/launchbadge/sqlx/pull/1894
[#1895]: https://github.com/launchbadge/sqlx/pull/1895
[#1897]: https://github.com/launchbadge/sqlx/pull/1897
[#1901]: https://github.com/launchbadge/sqlx/pull/1901

## 0.5.13 - 2022-04-15

This is a hotfix that reverts [#1748] as that was an accidental breaking change:  
the generated `PgHasArrayType` impl conflicts with manual impls of the trait.  
This change will have to wait for 0.6.0.

## 0.5.12 - 2022-04-13 (Yanked; use 0.5.13)
[27 pull requests][0.5.12-prs] were merged this release cycle.

### Added
* [[#1641]]: Postgres: Convenient wrapper for advisory locks [[@abonander]]
* [[#1675]]: Add function to undo migrations [[@jdrouet]]
* [[#1722]]: Postgres: implement `PgHasArrayType` for `serde_json::{Value, RawValue}` [[@abreis]]
* [[#1736]]: Derive `Clone` for `MySqlArguments` and `MssqlArguments` [[@0xdeafbeef]]
* [[#1748]]: Derive `PgHasArrayType` for `#[sqlx(transparent)]` types [[@carols10cents]]
* [[#1754]]: Include affected rows alongside returned rows in query logging [[@david-mcgillicuddy-moixa]]
* [[#1757]]: Implement `Type` for `Cow<str>` for MySQL, MSSQL and SQLite [[@ipetkov]]
* [[#1769]]: sqlx-cli: add `--source` to migration subcommands [[@pedromfedricci]]
* [[#1774]]: Postgres: make `extra_float_digits` settable [[@abonander]]
    * Can be set to `None` for Postgres or third-party database servers that don't support the option.
* [[#1776]]: Implement close-event notification for Pool [[@abonander]]
    * Also fixes `PgListener` preventing `Pool::close()` from resolving.
* [[#1780]]: Implement query builder [[@crajcan]]
    * See also [[#1790]]: Document and expand query builder [[@abonander]]
* [[#1781]]: Postgres: support `NUMERIC[]` using `decimal` feature [[@tm-drtina]]
* [[#1784]]: SQLite: add `FromStr`, `Copy`, `PartialEq`, `Eq` impls for options enums [[@andrewwhitehead]]

### Changed
* [[#1625]]: Update RustCrypto crates [[@paolobarbolini]]
* [[#1725]]: Update `heck` to 0.4 [[@paolobarbolini]]
* [[#1738]]: Update `regex` [[@Dylan-DPC]]
* [[#1763]]: SQLite: update `libsqlite3-sys` [[@espindola]]


### Fixed
* [[#1719]]: Fix a link in `query!()` docs [[@vbmade2000]]
* [[#1731]]: Postgres: fix option passing logic [[@liushuyu]]
* [[#1735]]: sqlx-cli: pass `DATABASE_URL` to command spawned in `prepare` [[@LovecraftianHorror]]
* [[#1741]]: Postgres: fix typo in `TSTZRANGE` [[@mgrachev]]
* [[#1761]]: Fix link from `QueryAs` to `query_as()` in docs [[@mgrachev]]
* [[#1786]]: MySQL: silence compile warnings for unused fields [[@andrewwhitehead]]
* [[#1789]]: SQLite: fix left-joins breaking `query!()` macros [[@tyrelr]]
* [[#1791]]: Postgres: fix newline parsing of `.pgpass` files [[@SebastienGllmt]]
* [[#1799]]: `PoolConnection`: don't leak connection permit if drop task fails to run [[@abonander]]

[#1625]: https://github.com/launchbadge/sqlx/pull/1625
[#1641]: https://github.com/launchbadge/sqlx/pull/1641
[#1675]: https://github.com/launchbadge/sqlx/pull/1675
[#1719]: https://github.com/launchbadge/sqlx/pull/1719
[#1722]: https://github.com/launchbadge/sqlx/pull/1722
[#1725]: https://github.com/launchbadge/sqlx/pull/1725
[#1731]: https://github.com/launchbadge/sqlx/pull/1731
[#1735]: https://github.com/launchbadge/sqlx/pull/1735
[#1736]: https://github.com/launchbadge/sqlx/pull/1736
[#1738]: https://github.com/launchbadge/sqlx/pull/1738
[#1741]: https://github.com/launchbadge/sqlx/pull/1741
[#1748]: https://github.com/launchbadge/sqlx/pull/1748
[#1754]: https://github.com/launchbadge/sqlx/pull/1754
[#1757]: https://github.com/launchbadge/sqlx/pull/1757
[#1761]: https://github.com/launchbadge/sqlx/pull/1761
[#1763]: https://github.com/launchbadge/sqlx/pull/1763
[#1769]: https://github.com/launchbadge/sqlx/pull/1769
[#1774]: https://github.com/launchbadge/sqlx/pull/1774
[#1776]: https://github.com/launchbadge/sqlx/pull/1776
[#1780]: https://github.com/launchbadge/sqlx/pull/1780
[#1781]: https://github.com/launchbadge/sqlx/pull/1781
[#1784]: https://github.com/launchbadge/sqlx/pull/1784
[#1786]: https://github.com/launchbadge/sqlx/pull/1786
[#1789]: https://github.com/launchbadge/sqlx/pull/1789
[#1790]: https://github.com/launchbadge/sqlx/pull/1790
[#1791]: https://github.com/launchbadge/sqlx/pull/1791
[#1799]: https://github.com/launchbadge/sqlx/pull/1799

[0.5.12-prs]: https://github.com/launchbadge/sqlx/pulls?q=is%3Apr+is%3Amerged+merged%3A2022-02-19..2022-04-13

## 0.5.11 - 2022-02-17
[20 pull requests][0.5.11-prs] were merged this release cycle.

### Added
* [[#1610]]: Allow converting `AnyConnectOptions` to a specific `ConnectOptions` [[@05storm26]]
* [[#1652]]: Implement `From` for `AnyConnection` [[@genusistimelord]]
* [[#1658]]: Handle `SQLITE_LOCKED` [[@madadam]]
* [[#1665]]: Document offline mode usage with feature flags [[@sedrik]]
* [[#1680]]: Show checksum mismatches in `sqlx migrate info` [[@ifn3]]
* [[#1685]]: Add tip for setting `opt-level` for `sqlx-macros` [[@LovecraftianHorror]]
* [[#1687]]: Docs: `Acquire` examples and alternative [[@stoically]]
* [[#1696]]: Postgres: support for `ltree` [[@cemoktra]]
* [[#1710]]: Postgres: support for `lquery` [[@cemoktra]]

### Changed
* [[#1605]]: Remove unused dependencies [[@paolobarbolini]]
* [[#1606]]: Add target context to Postgres `NOTICE` logs [[@dbeckwith]]
* [[#1684]]: Macros: Cache parsed `sqlx-data.json` instead of reparsing [[@LovecraftianHorror]]

### Fixed
* [[#1608]]: Drop worker shared state in shutdown (SQLite) [[@andrewwhitehead]]
* [[#1619]]: Docs(macros): remove sentences banning usage of `as _` [[@k-jun]]
* [[#1626]]: Simplify `cargo-sqlx` command-line definition [[@tranzystorek-io]]
* [[#1636]]: Fix and extend Postgres transaction example [[@taladar]]
* [[#1657]]: Fix typo in macro docs [[@p9s]]
* [[#1661]]: Fix binding `Option<T>` for `Any` driver [[@ArGGu]]
* [[#1667]]: MySQL: Avoid panicking if packet is empty [[@nappa85]]
* [[#1692]]: Postgres: Fix power calculation when encoding `BigDecimal` into `NUMERIC` [[@VersBinarii]]

Additionally, we have introduced two mitigations for [the issue of the cyclic dependency on `ahash`][aHash#95]: 

* We re-downgraded our version requirement on `indexmap` from `1.7.0` back to `1.6.2` so users can pin it to that
  version [as recommended in aHash#95][ahash-fix]. 
  * [This was regressed accidentally during a sweeping dependency upgrade before the last release][indexmap-regression],
    sorry about that.
* Thanks to the work of [@LovecraftianHorror] in [#1684], we no longer require the `preserve_order` feature of
  `serde_json` which gives users another place to break the cycle by simply not enabling that feature. 
  * This may introduce extra churn in Git diffs for `sqlx-data.json`, however. If this is an issue for you but 
    the dependency cycle isn't, you can re-enable the `preserve_order` feature:
  ```toml
  [dependencies]
  serde_json = { version = "1", features = ["preserve_order"] }
  ```

[aHash#95]: https://github.com/tkaitchuck/aHash/issues/95
[ahash-fix]: https://github.com/tkaitchuck/aHash/issues/95#issuecomment-874150078
[indexmap-regression]: https://github.com/launchbadge/sqlx/pull/1603#issuecomment-1010827637

[#1605]: https://github.com/launchbadge/sqlx/pull/1605
[#1606]: https://github.com/launchbadge/sqlx/pull/1606
[#1608]: https://github.com/launchbadge/sqlx/pull/1608
[#1610]: https://github.com/launchbadge/sqlx/pull/1610
[#1619]: https://github.com/launchbadge/sqlx/pull/1619
[#1626]: https://github.com/launchbadge/sqlx/pull/1626
[#1636]: https://github.com/launchbadge/sqlx/pull/1636
[#1652]: https://github.com/launchbadge/sqlx/pull/1652
[#1657]: https://github.com/launchbadge/sqlx/pull/1657
[#1658]: https://github.com/launchbadge/sqlx/pull/1658
[#1661]: https://github.com/launchbadge/sqlx/pull/1661
[#1665]: https://github.com/launchbadge/sqlx/pull/1665
[#1667]: https://github.com/launchbadge/sqlx/pull/1667
[#1680]: https://github.com/launchbadge/sqlx/pull/1680
[#1684]: https://github.com/launchbadge/sqlx/pull/1684
[#1685]: https://github.com/launchbadge/sqlx/pull/1685
[#1687]: https://github.com/launchbadge/sqlx/pull/1687
[#1692]: https://github.com/launchbadge/sqlx/pull/1692
[#1696]: https://github.com/launchbadge/sqlx/pull/1696
[#1710]: https://github.com/launchbadge/sqlx/pull/1710

[0.5.11-prs]: https://github.com/launchbadge/sqlx/pulls?q=is%3Apr+is%3Amerged+merged%3A2021-12-30..2022-02-17

## 0.5.10 - 2021-12-29
[A whopping 31 pull requests][0.5.10-prs] were merged this release cycle!

According to this changelog, we saw 18 new contributors! However, some of these folks may have missed getting
mentioned in previous entries since we only listed highlights. To avoid anyone feeling left out, I put in the effort
this time and tried to list every single one here.

### Added
* [[#1228]]: Add `Pool::any_kind()` [[@nitnelave]]
* [[#1343]]: Add `Encode/Decode` impl for `Cow<'_, str>` [[@Drevoed]]
* [[#1474]]: Derive `Clone`, `Copy` for `AnyKind` [[@yuyawk]]
* [[#1497]]: Update FAQ to explain how to configure docs.rs to build a project using SQLx [[@russweas]]
* [[#1498]]: Add description of migration file structure to `migrate!()` docs [[@zbigniewzolnierowicz]]
* [[#1508]]: Add `.persistent(bool)` to `QueryAs`, `QueryScalar` [[@akiradeveloper]]
* [[#1514]]: Add support for serialized threading mode to SQLite [[@LLBlumire]]
* [[#1523]]: Allow `rust_decimal::Decimal` in `PgRange` [[@meh]]
* [[#1539]]: Support `PGOPTIONS` and adding custom configuration options in `PgConnectOptions` [[@liushuyu]]
* [[#1562]]: Re-export `either::Either` used by `Executor::fetch_many()` [[@DoumanAsh]]
* [[#1584]]: Add feature to use RusTLS instead of `native-tls` for `sqlx-cli` [[@SonicZentropy]]
* [[#1592]]: Add `AnyConnection::kind()` [[@05storm26]]

### Changes
* [[#1385]]: Rewrite Postgres array handling to reduce boilerplate and allow custom types [[@jplatte]]
* [[#1479]]: Remove outdated mention of `runtime-async-std-native-tls` as the default runtime in README.md [[@yerke]]
* [[#1526]]: Revise `Pool` docs in a couple places [[@abonander]]
* [[#1535]]: Bump `libsqlite-sys` to `0.23.1` [[@nitsky]]
* [[#1551]]: SQLite: make worker thread responsible for all FFI calls [[@abonander]]
    * If you were encountering segfaults with the SQLite driver, there's a good chance this will fix it!
* [[#1557]]: CI: test with Postgres 14 [[@paolobarbolini]]
* [[#1571]]: Make `whoami` dep optional, only pull it in for Postgres [[@joshtriplett]]
* [[#1572]]: Update `rsa` crate to 0.5 [[@paolobarbolini]]
* [[#1591]]: List SeaORM as an ORM option in the README [[@kunjee17]]
* [[#1601]]: Update `itoa` and `dirs` [[@paolobarbolini]]

### Fixes
* [[#1475]]: Fix panic when converting a negative `chrono::Duration` to `PgInterval` [[@yuyawk]]
* [[#1483]]: Fix error when decoding array of custom types from Postgres [[@demurgos]
* [[#1501]]: Reduce `indexmap` version requirement to `1.6.2` [[@dimfeld]]
* [[#1511]]: Fix element type given to Postgres for arrays of custom enums [[@chesedo]]
* [[#1517]]: Fix mismatched type errors in MySQL type tests [[@abonander]]
* [[#1537]]: Fix missing re-export of `PgCopyIn` [[@akiradeveloper]]
* [[#1566]]: Match `~/.pgpass` password after URL parsing and fix user and database ordering [[@D1plo1d]]
* [[#1582]]: `cargo sqlx prepare`: Append to existing `RUSTFLAGS` instead of overwriting [[@tkintscher]]
* [[#1587]]: SQLite: if set, send `PRAGMA key` on a new connection before anything else. [[@parazyd]]
    * This should fix problems with being unable to open databases using SQLCipher.
    

[#1228]: https://github.com/launchbadge/sqlx/pull/1228
[#1343]: https://github.com/launchbadge/sqlx/pull/1343
[#1385]: https://github.com/launchbadge/sqlx/pull/1385
[#1474]: https://github.com/launchbadge/sqlx/pull/1474
[#1475]: https://github.com/launchbadge/sqlx/pull/1475
[#1479]: https://github.com/launchbadge/sqlx/pull/1479
[#1483]: https://github.com/launchbadge/sqlx/pull/1483
[#1497]: https://github.com/launchbadge/sqlx/pull/1497
[#1498]: https://github.com/launchbadge/sqlx/pull/1498
[#1501]: https://github.com/launchbadge/sqlx/pull/1501
[#1508]: https://github.com/launchbadge/sqlx/pull/1508 
[#1511]: https://github.com/launchbadge/sqlx/pull/1511
[#1514]: https://github.com/launchbadge/sqlx/pull/1514
[#1517]: https://github.com/launchbadge/sqlx/pull/1517
[#1523]: https://github.com/launchbadge/sqlx/pull/1523
[#1526]: https://github.com/launchbadge/sqlx/pull/1526
[#1535]: https://github.com/launchbadge/sqlx/pull/1535
[#1537]: https://github.com/launchbadge/sqlx/pull/1537
[#1539]: https://github.com/launchbadge/sqlx/pull/1539
[#1551]: https://github.com/launchbadge/sqlx/pull/1551
[#1557]: https://github.com/launchbadge/sqlx/pull/1557
[#1562]: https://github.com/launchbadge/sqlx/pull/1562
[#1566]: https://github.com/launchbadge/sqlx/pull/1566
[#1571]: https://github.com/launchbadge/sqlx/pull/1571
[#1572]: https://github.com/launchbadge/sqlx/pull/1572
[#1582]: https://github.com/launchbadge/sqlx/pull/1582
[#1584]: https://github.com/launchbadge/sqlx/pull/1584
[#1587]: https://github.com/launchbadge/sqlx/pull/1587
[#1591]: https://github.com/launchbadge/sqlx/pull/1591
[#1592]: https://github.com/launchbadge/sqlx/pull/1592
[#1601]: https://github.com/launchbadge/sqlx/pull/1601
[0.5.10-prs]: https://github.com/launchbadge/sqlx/pulls?page=1&q=is%3Apr+merged%3A2021-10-02..2021-12-31+sort%3Acreated-asc

## 0.5.9 - 2021-10-01

A hotfix release to address the issue of the `sqlx` crate itself still depending on older versions of `sqlx-core` and 
`sqlx-macros`.

No other changes from `0.5.8`.

## 0.5.8 - 2021-10-01 (Yanked; use 0.5.9)

[A total of 24 pull requests][0.5.8-prs] were merged this release cycle! Some highlights: 

* [[#1289]] Support the `immutable` option on SQLite connections [[@djmarcin]]
* [[#1295]] Support custom initial options for SQLite [[@ghassmo]]
    * Allows specifying custom `PRAGMA`s and overriding those set by SQLx.
* [[#1345]] Initial support for Postgres `COPY FROM/TO`[[@montanalow], [@abonander]]
* [[#1439]] Handle multiple waiting results correctly in MySQL [[@eagletmt]]

[#1289]: https://github.com/launchbadge/sqlx/pull/1289
[#1295]: https://github.com/launchbadge/sqlx/pull/1295
[#1345]: https://github.com/launchbadge/sqlx/pull/1345
[#1439]: https://github.com/launchbadge/sqlx/pull/1439
[0.5.8-prs]: https://github.com/launchbadge/sqlx/pulls?q=is%3Apr+is%3Amerged+merged%3A2021-08-21..2021-10-01

## 0.5.7 - 2021-08-20

* [[#1392]] use `resolve_path` when getting path for `include_str!()` [[@abonander]]
    * Fixes a regression introduced by [[#1332]].
* [[#1393]] avoid recursively spawning tasks in `PgListener::drop()` [[@abonander]]
    * Fixes a panic that occurs when `PgListener` is dropped in `async fn main()`.

[#1392]: https://github.com/launchbadge/sqlx/pull/1392
[#1393]: https://github.com/launchbadge/sqlx/pull/1393

## 0.5.6 - 2021-08-16

A large bugfix release, including but not limited to:

* [[#1329]] Implement `MACADDR` type for Postgres [[@nomick]]
* [[#1363]] Fix `PortalSuspended` for array of composite types in Postgres [[@AtkinsChang]]
* [[#1320]] Reimplement `sqlx::Pool` internals using `futures-intrusive` [[@abonander]]
    * This addresses a number of deadlocks/stalls on acquiring connections from the pool.
* [[#1332]] Macros: tell the compiler about external files/env vars to watch [[@abonander]]
    * Includes `sqlx build-script` to create a `build.rs` to watch `migrations/` for changes.
    * Nightly users can try `RUSTFLAGS=--cfg sqlx_macros_unstable` to tell the compiler 
      to watch `migrations/` for changes instead of using a build script. 
    * See the new section in the docs for `sqlx::migrate!()` for details.
* [[#1351]] Fix a few sources of segfaults/errors in SQLite driver [[@abonander]]
    * Includes contributions from [[@link2ext]] and [[@madadam]].
* [[#1323]] Keep track of column typing in SQLite EXPLAIN parsing [[@marshoepial]]
    * This fixes errors in the macros when using `INSERT/UPDATE/DELETE ... RETURNING ...` in SQLite.
    
[A total of 25 pull requests][0.5.6-prs] were merged this release cycle!

[#1329]: https://github.com/launchbadge/sqlx/pull/1329
[#1363]: https://github.com/launchbadge/sqlx/pull/1363
[#1320]: https://github.com/launchbadge/sqlx/pull/1320
[#1332]: https://github.com/launchbadge/sqlx/pull/1332
[#1351]: https://github.com/launchbadge/sqlx/pull/1351
[#1323]: https://github.com/launchbadge/sqlx/pull/1323
[0.5.6-prs]: https://github.com/launchbadge/sqlx/pulls?q=is%3Apr+is%3Amerged+merged%3A2021-05-24..2021-08-17

## 0.5.5 - 2021-05-24

-   [[#1242]] Fix infinite loop at compile time when using query macros [[@toshokan]]

[#1242]: https://github.com/launchbadge/sqlx/pull/1242

## 0.5.4 - 2021-05-22

-   [[#1235]] Fix compilation with rustls from an eager update to webpki [[@ETCaton]]

[#1235]: https://github.com/launchbadge/sqlx/pull/1235

## 0.5.3 - 2021-05-21

-   [[#1211]] Even more tweaks and fixes to the Pool internals [[@abonander]]

-   [[#1213]] Add support for bytes and `chrono::NaiveDateTime` to `Any` [[@guylapid]]

-   [[#1224]] Add support for `chrono::DateTime<Local>` to `Any` with `MySQL` [[@NatPRoach]]

-   [[#1216]] Skip empty lines and comments in pgpass files [[@feikesteenbergen]]

-   [[#1218]] Add support for `PgMoney` to the compile-time type-checking [[@iamsiddhant05]]

[#1211]: https://github.com/launchbadge/sqlx/pull/1211
[#1213]: https://github.com/launchbadge/sqlx/pull/1213
[#1216]: https://github.com/launchbadge/sqlx/pull/1216
[#1218]: https://github.com/launchbadge/sqlx/pull/1218
[#1224]: https://github.com/launchbadge/sqlx/pull/1224

## 0.5.2 - 2021-04-15

-   [[#1149]] Tweak and optimize Pool internals [[@abonander]]

-   [[#1132]] Remove `'static` bound on `Connection::transaction` [[@argv-minus-one]]

-   [[#1128]] Fix `-y` flag for `sqlx db reset -y` [[@qqwa]]

-   [[#1099]] [[#1097]] Truncate buffer when `BufStream` is dropped [[@Diggsey]]

[#1132]: https://github.com/launchbadge/sqlx/pull/1132
[#1149]: https://github.com/launchbadge/sqlx/pull/1149
[#1128]: https://github.com/launchbadge/sqlx/pull/1128
[#1099]: https://github.com/launchbadge/sqlx/pull/1099
[#1097]: https://github.com/launchbadge/sqlx/issues/1097

### PostgreSQL

-   [[#1170]] Remove `Self: Type` bounds in `Encode` / `Decode` implementations for arrays [[@jplatte]]

    Enables working around the lack of support for user-defined array types:

    ```rust
    #[derive(sqlx::Encode)]
    struct Foos<'a>(&'a [Foo]);

    impl sqlx::Type<sqlx::Postgres> for Foos<'_> {
        fn type_info() -> PgTypeInfo {
            PgTypeInfo::with_name("_foo")
        }
    }

    query_as!(
        Whatever,
        "<QUERY with $1 of type foo[]>",
        Foos(&foo_vec) as _,
    )
    ```

-   [[#1141]] Use `u16::MAX` instead of `i16::MAX` for a check against the largest number of parameters in a query [[@crajcan]]

-   [[#1112]] Add support for `DOMAIN` types [[@demurgos]]

-   [[#1100]] Explicitly `UNLISTEN` before returning connections to the pool in `PgListener` [[@Diggsey]]

[#1170]: https://github.com/launchbadge/sqlx/pull/1170
[#1141]: https://github.com/launchbadge/sqlx/pull/1141
[#1112]: https://github.com/launchbadge/sqlx/pull/1112
[#1100]: https://github.com/launchbadge/sqlx/pull/1100

### SQLite

-   [[#1161]] Catch `SQLITE_MISUSE` on connection close and panic [[@link2xt]]

-   [[#1160]] Do not cast pointers to `i32` (cast to `usize`) [[@link2xt]]

-   [[#1156]] Reset the statement when `fetch_many` stream is dropped [[@link2xt]]

[#1161]: https://github.com/launchbadge/sqlx/pull/1161
[#1160]: https://github.com/launchbadge/sqlx/pull/1160
[#1156]: https://github.com/launchbadge/sqlx/pull/1156

## 0.5.1 - 2021-02-04

-   Update sqlx-rt to 0.3.

## 0.5.0 - 2021-02-04

### Changes

-   [[#983]] [[#1022]] Upgrade async runtime dependencies [[@seryl], [@ant32], [@jplatte], [@robjtede]]

    -   tokio 1.0
    -   actix-rt 2.0

-   [[#854]] Allow chaining `map` and `try_map` [[@jplatte]]

    Additionally enables calling these combinators with the macros:

    ```rust
    let ones: Vec<i32> = query!("SELECT 1 as foo")
        .map(|row| row.foo)
        .fetch_all(&mut conn).await?;
    ```

-   [[#940]] Rename the `#[sqlx(rename)]` attribute used to specify the type name on the database
    side to `#[sqlx(type_name)]` [[@jplatte]].

-   [[#976]] Rename the `DbDone` types to `DbQueryResult`. [[@jplatte]]

-   [[#976]] Remove the `Done` trait. The `.rows_affected()` method is now available as an inherent
    method on `PgQueryResult`, `MySqlQueryResult` and so on. [[@jplatte]]

-   [[#1007]] Remove `any::AnyType` (and replace with directly implementing `Type<Any>`) [[@jplatte]]

### Added

-   [[#998]] [[#821]] Add `.constraint()` to `DatabaseError` [[@fl9]]

-   [[#919]] For SQLite, add support for unsigned integers [[@dignifiedquire]]

### Fixes

-   [[#1002]] For SQLite, `GROUP BY` in `query!` caused an infinite loop at compile time. [[@pymongo]]

-   [[#979]] For MySQL, fix support for non-default authentication. [[@sile]]

-   [[#918]] Recover from dropping `wait_for_conn` inside Pool. [[@antialize]]

[#821]: https://github.com/launchbadge/sqlx/issues/821
[#918]: https://github.com/launchbadge/sqlx/pull/918
[#919]: https://github.com/launchbadge/sqlx/pull/919
[#983]: https://github.com/launchbadge/sqlx/pull/983
[#940]: https://github.com/launchbadge/sqlx/pull/940
[#976]: https://github.com/launchbadge/sqlx/pull/976
[#979]: https://github.com/launchbadge/sqlx/pull/979
[#998]: https://github.com/launchbadge/sqlx/pull/998
[#983]: https://github.com/launchbadge/sqlx/pull/983
[#1002]: https://github.com/launchbadge/sqlx/pull/1002
[#1007]: https://github.com/launchbadge/sqlx/pull/1007
[#1022]: https://github.com/launchbadge/sqlx/pull/1022

## 0.4.2 - 2020-12-19

-   [[#908]] Fix `whoami` crash on FreeBSD platform [[@fundon]] [[@AldaronLau]]

-   [[#895]] Decrement pool size when connection is released [[@andrewwhitehead]]

-   [[#878]] Fix `conn.transaction` wrapper [[@hamza1311]]

    ```rust
    conn.transaction(|transaction: &mut Transaction<Database> | {
        // ...
    });
    ```

-   [[#874]] Recognize `1` as `true` for `SQLX_OFFLINE [[@Pleto]]

-   [[#747]] [[#867]] Replace `lru-cache` with `hashlink` [[@chertov]]

-   [[#860]] Add `rename_all` to `FromRow` and add `camelCase` and `PascalCase` [[@framp]]

-   [[#839]] Add (optional) support for `bstr::BStr`, `bstr::BString`, and `git2::Oid` [[@joshtriplett]]

#### SQLite

-   [[#893]] Fix memory leak if `create_collation` fails [[@slumber]]

-   [[#852]] Fix potential 100% CPU usage in `fetch_one` / `fetch_optional` [[@markazmierczak]]

-   [[#850]] Add `synchronous` option to `SqliteConnectOptions` [[@markazmierczak]]

#### PostgreSQL

-   [[#889]] Fix decimals (one more time) [[@slumber]]

-   [[#876]] Add support for `BYTEA[]` to compile-time type-checking [[@augustocdias]]

-   [[#845]] Fix path for `&[NaiveTime]` in `query!` macros [[@msrd0]]

#### MySQL

-   [[#880]] Consider `utf8mb4_general_ci` as a string [[@mcronce]]

[#908]: https://github.com/launchbadge/sqlx/pull/908
[#895]: https://github.com/launchbadge/sqlx/pull/895
[#893]: https://github.com/launchbadge/sqlx/pull/893
[#889]: https://github.com/launchbadge/sqlx/pull/889
[#880]: https://github.com/launchbadge/sqlx/pull/880
[#878]: https://github.com/launchbadge/sqlx/pull/878
[#876]: https://github.com/launchbadge/sqlx/pull/876
[#874]: https://github.com/launchbadge/sqlx/pull/874
[#867]: https://github.com/launchbadge/sqlx/pull/867
[#860]: https://github.com/launchbadge/sqlx/pull/860
[#854]: https://github.com/launchbadge/sqlx/pull/854
[#852]: https://github.com/launchbadge/sqlx/pull/852
[#850]: https://github.com/launchbadge/sqlx/pull/850
[#845]: https://github.com/launchbadge/sqlx/pull/845
[#839]: https://github.com/launchbadge/sqlx/pull/839
[#747]: https://github.com/launchbadge/sqlx/issues/747

## 0.4.1 – 2020-11-13

Fix docs.rs build by enabling a runtime feature in the docs.rs metadata in `Cargo.toml`.

## 0.4.0 - 2020-11-12

-   [[#774]] Fix usage of SQLx derives with other derive crates [[@NyxCode]]

-   [[#762]] Fix `migrate!()` (with no params) [[@esemeniuc]]

-   [[#755]] Add `kebab-case` to `rename_all` [[@iamsiddhant05]]

-   [[#735]] Support `rustls` [[@jplatte]]

    Adds `-native-tls` or `-rustls` on each runtime feature:

    ```toml
    # previous
    features = [ "runtime-async-std" ]

    # now
    features = [ "runtime-async-std-native-tls" ]
    ```

-   [[#718]] Support tuple structs with `#[derive(FromRow)]` [[@dvermd]]

#### SQLite

-   [[#789]] Support `$NNN` parameters [[@nitsky]]

-   [[#784]] Use `futures_channel::oneshot` in worker for big perf win [[@markazmierczak]]

#### PostgreSQL

-   [[#781]] Fix decimal conversions handling of `0.01` [[@pimeys]]

-   [[#745]] Always prefer parsing of the non-localized notice severity field [[@dstoeckel]]

-   [[#742]] Enable `Vec<DateTime<Utc>>` with chrono [[@mrcd]]

#### MySQL

-   [[#743]] Consider `utf8mb4_bin` as a string [[@digorithm]]

-   [[#739]] Fix minor protocol detail with `iteration-count` that was blocking Vitess [[@mcronce]]

[#774]: https://github.com/launchbadge/sqlx/pull/774
[#789]: https://github.com/launchbadge/sqlx/pull/789
[#784]: https://github.com/launchbadge/sqlx/pull/784
[#781]: https://github.com/launchbadge/sqlx/pull/781
[#762]: https://github.com/launchbadge/sqlx/pull/762
[#755]: https://github.com/launchbadge/sqlx/pull/755
[#745]: https://github.com/launchbadge/sqlx/pull/745
[#743]: https://github.com/launchbadge/sqlx/pull/743
[#742]: https://github.com/launchbadge/sqlx/pull/742
[#735]: https://github.com/launchbadge/sqlx/pull/735
[#739]: https://github.com/launchbadge/sqlx/pull/739
[#718]: https://github.com/launchbadge/sqlx/pull/718

## 0.4.0-beta.1 - 2020-07-27

### Highlights

-   Enable compile-time type checking from cached metadata to enable building
    in an environment without access to a development database (e.g., Docker, CI).

-   Initial support for **Microsoft SQL Server**. If there is something missing that you need,
    open an issue. We are happy to help.

-   SQL migrations, both with a CLI tool and programmatically loading migrations at runtime.

-   Runtime-determined database driver, `Any`, to support compile-once and run with a database
    driver selected at runtime.

-   Support for user-defined types and more generally overriding the inferred Rust type from SQL
    with compile-time SQL verification.

### Fixed

#### MySQL

-   [[#418]] Support zero dates and times [[@blackwolf12333]]

### Added

-   [[#174]] Inroduce a builder to construct connections to bypass the URL parsing

    ```rust
    // MSSQL
    let conn = MssqlConnectOptions::new()
        .host("localhost")
        .database("master")
        .username("sa")
        .password("Password")
        .connect().await?;

    // SQLite
    let conn = SqliteConnectOptions::from_str("sqlite://a.db")?
        .foreign_keys(false)
        .connect().await?;
    ```

-   [[#127]] Get the last ID or Row ID inserted for MySQL or SQLite

    ```rust
    // MySQL
    let id: u64 = query!("INSERT INTO table ( col ) VALUES ( ? )", val)
        .execute(&mut conn).await?
        .last_insert_id(); // LAST_INSERT_ID()

    // SQLite
    let id: i64 = query!("INSERT INTO table ( col ) VALUES ( ?1 )", val)
        .execute(&mut conn).await?
        .last_insert_rowid(); // sqlite3_last_insert_rowid()
    ```

-   [[#263]] Add hooks to the Pool: `after_connect`, `before_release`, and `after_acquire`

    ```rust
    // PostgreSQL
    let pool = PgPoolOptions::new()
        .after_connect(|conn| Box::pin(async move {
            conn.execute("SET application_name = 'your_app';").await?;
            conn.execute("SET search_path = 'my_schema';").await?;

            Ok(())
        }))
        .connect("postgres:// …").await?
    ```

-   [[#308]] [[#495]] Extend `derive(FromRow)` with support for `#[sqlx(default)]` on fields to allow reading in a partial query [[@OriolMunoz]]

-   [[#454]] [[#456]] Support `rust_decimal::Decimal` as an alternative to `bigdecimal::BigDecimal` for `NUMERIC` columns in MySQL and PostgreSQL [[@pimeys]]

-   [[#181]] Column names and type information is now accessible from `Row` via `Row::columns()` or `Row::column(name)`

#### PostgreSQL

-   [[#197]] [[#271]] Add initial support for `INTERVAL` (full support pending a `time::Period` type) [[@dimtion]]

#### MySQL

-   [[#449]] [[#450]] Support Unix Domain Sockets (UDS) for MySQL [[@pimeys]]

#### SQLite

-   Types are now inferred for expressions. This means its now possible to use `query!` and `query_as!` for:

    ```rust
    let row = query!("SELECT 10 as _1, x + 5 as _2 FROM table").fetch_one(&mut conn).await?;

    assert_eq!(row._1, 10);
    assert_eq!(row._2, 5); // 5 + x?
    ```

-   [[#167]] Support `foreign_keys` explicitly with a `foreign_keys(true)` method available on `SqliteConnectOptions` which is a builder
    for new SQLite connections (and can be passed into `PoolOptions` to build a pool).

    ```rust
    let conn = SqliteConnectOptions::new()
        .foreign_keys(true) // on by default
        .connect().await?;
    ```

-   [[#430]] [[#438]] Add method to get the raw SQLite connection handle [[@agentsim]]

    ```rust
    // conn is `SqliteConnection`
    // this is not unsafe, but what you do with the handle will be
    let ptr: *mut libsqlite3::sqlite3 = conn.as_raw_handle();
    ```

-   [[#164]] Support `TIMESTAMP`, `DATETIME`, `DATE`, and `TIME` via `chrono` in SQLite [[@felipesere]] [[@meteficha]]

### Changed

-   `Transaction` now mutably borrows a connection instead of owning it. This enables a new (or nested) transaction to be started from `&mut conn`.

-   [[#145]] [[#444]] Use a least-recently-used (LRU) cache to limit the growth of the prepared statement cache for SQLite, MySQL, and PostgreSQL [[@pimeys]]

#### SQLite

-   [[#499]] `INTEGER` now resolves to `i64` instead of `i32`, `INT4` will still resolve to `i32`

### Removed

[#127]: https://github.com/launchbadge/sqlx/issues/127
[#174]: https://github.com/launchbadge/sqlx/issues/174
[#145]: https://github.com/launchbadge/sqlx/issues/145
[#164]: https://github.com/launchbadge/sqlx/issues/164
[#167]: https://github.com/launchbadge/sqlx/issues/167
[#181]: https://github.com/launchbadge/sqlx/issues/181
[#197]: https://github.com/launchbadge/sqlx/issues/197
[#263]: https://github.com/launchbadge/sqlx/issues/263
[#308]: https://github.com/launchbadge/sqlx/issues/308
[#418]: https://github.com/launchbadge/sqlx/issues/418
[#430]: https://github.com/launchbadge/sqlx/issues/430
[#449]: https://github.com/launchbadge/sqlx/issues/449
[#499]: https://github.com/launchbadge/sqlx/issues/499
[#454]: https://github.com/launchbadge/sqlx/issues/454
[#271]: https://github.com/launchbadge/sqlx/pull/271
[#444]: https://github.com/launchbadge/sqlx/pull/444
[#438]: https://github.com/launchbadge/sqlx/pull/438
[#495]: https://github.com/launchbadge/sqlx/pull/495
[#495]: https://github.com/launchbadge/sqlx/pull/495

## 0.3.5 - 2020-05-06

### Fixed

-   [[#259]] Handle percent-encoded paths for SQLite [[@g-s-k]]

-   [[#281]] Deallocate SQLite statements before closing the SQLite connection [[@hasali19]]

-   [[#284]] Fix handling of `0` for `BigDecimal` in PostgreSQL and MySQL [[@abonander]]

### Added

-   [[#256]] Add `query_unchecked!` and `query_file_unchecked!` with similar semantics to `query_as_unchecked!` [[@meh]]

-   [[#252]] [[#297]] Derive several traits for the `Json<T>` wrapper type [[@meh]]

-   [[#261]] Add support for `#[sqlx(rename_all = "snake_case")]` to `#[derive(Type)]` [[@shssoichiro]]

-   [[#253]] Add support for UNIX domain sockets to PostgreSQL [[@Nilix007]]

-   [[#251]] Add support for textual JSON on MySQL [[@blackwolf12333]]

-   [[#275]] [[#268]] Optionally log formatted SQL queries on execution [[@shssoichiro]]

-   [[#267]] Support Cargo.toml relative `.env` files; allows for each crate in a workspace to use their own `.env` file and thus their own `DATABASE_URL` [[@xyzd]]

[#252]: https://github.com/launchbadge/sqlx/pull/252
[#261]: https://github.com/launchbadge/sqlx/pull/261
[#256]: https://github.com/launchbadge/sqlx/pull/256
[#259]: https://github.com/launchbadge/sqlx/pull/259
[#253]: https://github.com/launchbadge/sqlx/pull/253
[#297]: https://github.com/launchbadge/sqlx/pull/297
[#251]: https://github.com/launchbadge/sqlx/pull/251
[#275]: https://github.com/launchbadge/sqlx/pull/275
[#267]: https://github.com/launchbadge/sqlx/pull/267
[#268]: https://github.com/launchbadge/sqlx/pull/268
[#281]: https://github.com/launchbadge/sqlx/pull/281
[#284]: https://github.com/launchbadge/sqlx/pull/284

## 0.3.4 - 2020-04-10

### Fixed

-   [[#241]] Type name for custom enum is not always attached to TypeInfo in PostgreSQL

-   [[#237]] [[#238]] User-defined type name matching is now case-insensitive in PostgreSQL [[@qtbeee]]

-   [[#231]] Handle empty queries (and those with comments) in SQLite

-   [[#228]] Provide `MapRow` implementations for functions (enables `.map(|row| ...)` over `.try_map(|row| ...)`)

### Added

-   [[#234]] Add support for `NUMERIC` in MySQL with the `bigdecimal` crate [[@xiaopengli89]]

-   [[#227]] Support `#[sqlx(rename = "new_name")]` on struct fields within a `FromRow` derive [[@sidred]]

[#228]: https://github.com/launchbadge/sqlx/issues/228
[#231]: https://github.com/launchbadge/sqlx/issues/231
[#237]: https://github.com/launchbadge/sqlx/issues/237
[#241]: https://github.com/launchbadge/sqlx/issues/241
[#227]: https://github.com/launchbadge/sqlx/pull/227
[#234]: https://github.com/launchbadge/sqlx/pull/234
[#238]: https://github.com/launchbadge/sqlx/pull/238

## 0.3.3 - 2020-04-01

### Fixed

-   [[#214]] Handle percent-encoded usernames in a database URL [[@jamwaffles]]

### Changed

-   [[#216]] Mark `Cursor`, `Query`, `QueryAs`, `query::Map`, and `Transaction` as `#[must_use]` [[@Ace4896]]

-   [[#213]] Remove matches dependency and use matches macro from std [[@nrjais]]

[#216]: https://github.com/launchbadge/sqlx/pull/216
[#214]: https://github.com/launchbadge/sqlx/pull/214
[#213]: https://github.com/launchbadge/sqlx/pull/213

## 0.3.2 - 2020-03-31

### Fixed

-   [[#212]] Removed sneaky `println!` in `MySqlCursor`

[#212]: https://github.com/launchbadge/sqlx/issues/212

## 0.3.1 - 2020-03-30

### Fixed

-   [[#203]] Allow an empty password for MySQL

-   [[#204]] Regression in error reporting for invalid SQL statements on PostgreSQL

-   [[#200]] Fixes the incorrect handling of raw (`r#...`) fields of a struct in the `FromRow` derive [[@sidred]]

[#200]: https://github.com/launchbadge/sqlx/pull/200
[#203]: https://github.com/launchbadge/sqlx/issues/203
[#204]: https://github.com/launchbadge/sqlx/issues/204

## 0.3.0 - 2020-03-29

### Breaking Changes

-   `sqlx::Row` now has a lifetime (`'c`) tied to the database connection. In effect, this means that you cannot store `Row`s or collect
    them into a collection. `Query` (returned from `sqlx::query()`) has `map()` which takes a function to map from the `Row` to
    another type to make this transition easier.

    In 0.2.x

    ```rust
    let rows = sqlx::query("SELECT 1")
        .fetch_all(&mut conn).await?;
    ```

    In 0.3.x

    ```rust
    let values: Vec<i32> = sqlx::query("SELECT 1")
        .map(|row: PgRow| row.get(0))
        .fetch_all(&mut conn).await?;
    ```

    To assist with the above, `sqlx::query_as()` now supports querying directly into tuples (up to 9 elements) or
    struct types with a `#[derive(FromRow)]`.

    ```rust
    // This extension trait is needed until a rust bug is fixed
    use sqlx::postgres::PgQueryAs;

    let values: Vec<(i32, bool)> = sqlx::query_as("SELECT 1, false")
        .fetch_all(&mut conn).await?;
    ```

-   `HasSqlType<T>: Database` is now `T: Type<Database>` to mirror `Encode` and `Decode`

-   `Query::fetch` (returned from `query()`) now returns a new `Cursor` type. `Cursor` is a Stream-like type where the
    item type borrows into the stream (which itself borrows from connection). This means that using `query().fetch()` you can now
    stream directly from the database with **zero-copy** and **zero-allocation**.

-   Remove `PgTypeInfo::with_oid` and replace with `PgTypeInfo::with_name`

### Added

-   Results from the database are now zero-copy and no allocation beyond a shared read buffer
    for the TCP stream ( in other words, almost no per-query allocation ). Bind arguments still
    do allocate a buffer per query.

-   [[#129]] Add support for [SQLite](https://sqlite.org/index.html). Generated code should be very close to normal use of the C API.

    -   Adds `Sqlite`, `SqliteConnection`, `SqlitePool`, and other supporting types

-   [[#97]] [[#134]] Add support for user-defined types. [[@Freax13]]

    -   Rust-only domain types or transparent wrappers around SQL types. These may be used _transparently_ inplace of
        the SQL type.

        ```rust
        #[derive(sqlx::Type)]
        #[repr(transparent)]
        struct Meters(i32);
        ```

    -   Enumerations may be defined in Rust and can match SQL by integer discriminant or variant name.

        ```rust
        #[derive(sqlx::Type)]
        #[repr(i32)] // Expects a INT in SQL
        enum Color { Red = 1, Green = 2, Blue = 3 }
        ```

        ```rust
        #[derive(sqlx::Type)]
        #[sqlx(rename = "TEXT")] // May also be the name of a user defined enum type
        #[sqlx(rename_all = "lowercase")] // similar to serde rename_all
        enum Color { Red, Green, Blue } // expects 'red', 'green', or 'blue'
        ```

    -   **Postgres** further supports user-defined composite types.

        ```rust
        #[derive(sqlx::Type)]
        #[sqlx(rename = "interface_type")]
        struct InterfaceType {
            name: String,
            supplier_id: i32,
            price: f64
        }
        ```

-   [[#98]] [[#131]] Add support for asynchronous notifications in Postgres (`LISTEN` / `NOTIFY`). [[@thedodd]]

    -   Supports automatic reconnection on connection failure.

    -   `PgListener` implements `Executor` and may be used to execute queries. Be careful however as if the
        intent is to handle and process messages rapidly you don't want to be tying up the connection
        for too long. Messages received during queries are buffered and will be delivered on the next call
        to `recv()`.

    ```rust
    let mut listener = PgListener::new(DATABASE_URL).await?;

    listener.listen("topic").await?;

    loop {
        let message = listener.recv().await?;

        println!("payload = {}", message.payload);
    }
    ```

-   Add _unchecked_ variants of the query macros. These will still verify the SQL for syntactic and
    semantic correctness with the current database but they will not check the input or output types.

    This is intended as a temporary solution until `query_as!` is able to support user defined types.

    -   `query_as_unchecked!`
    -   `query_file_as_unchecked!`

-   Add support for many more types in Postgres

    -   `JSON`, `JSONB` [[@oeb25]]
    -   `INET`, `CIDR` [[@PoiScript]]
    -   Arrays [[@oeb25]]
    -   Composites ( Rust tuples or structs with a `#[derive(Type)]` )
    -   `NUMERIC` [[@abonander]]
    -   `OID` (`u32`)
    -   `"CHAR"` (`i8`)
    -   `TIMESTAMP`, `TIMESTAMPTZ`, etc. with the `time` crate [[@utter-step]]
    -   Enumerations ( Rust enums with a `#[derive(Type)]` ) [[@Freax13]]

### Changed

-   `Query` (and `QueryAs`; returned from `query()`, `query_as()`, `query!()`, and `query_as!()`) now will accept both `&mut Connection` or
    `&Pool` where as in 0.2.x they required `&mut &Pool`.

-   `Executor` now takes any value that implements `Execute` as a query. `Execute` is implemented for `Query` and `QueryAs` to mean
    exactly what they've meant so far, a prepared SQL query. However, `Execute` is also implemented for just `&str` which now performs
    a raw or unprepared SQL query. You can further use this to fetch `Row`s from the database though it is not as efficient as the
    prepared API (notably Postgres and MySQL send data back in TEXT mode as opposed to in BINARY mode).

    ```rust
    use sqlx::Executor;

    // Set the time zone parameter
    conn.execute("SET TIME ZONE LOCAL;").await

    // Demonstrate two queries at once with the raw API
    let mut cursor = conn.fetch("SELECT 1; SELECT 2");
    let row = cursor.next().await?.unwrap();
    let value: i32 = row.get(0); // 1
    let row = cursor.next().await?.unwrap();
    let value: i32 = row.get(0); // 2
    ```

### Removed

-   `Query` (returned from `query()`) no longer has `fetch_one`, `fetch_optional`, or `fetch_all`. You _must_ map the row using `map()` and then
    you will have a `query::Map` value that has the former methods available.

    ```rust
    let values: Vec<i32> = sqlx::query("SELECT 1")
        .map(|row: PgRow| row.get(0))
        .fetch_all(&mut conn).await?;
    ```

### Fixed

-   [[#62]] [[#130]] [[#135]] Remove explicit set of `IntervalStyle`. Allow usage of SQLx for CockroachDB and potentially PgBouncer. [[@bmisiak]]

-   [[#108]] Allow nullable and borrowed values to be used as arguments in `query!` and `query_as!`. For example, where the column would
    resolve to `String` in Rust (TEXT, VARCHAR, etc.), you may now use `Option<String>`, `Option<&str>`, or `&str` instead. [[@abonander]]

-   [[#108]] Make unknown type errors far more informative. As an example, trying to `SELECT` a `DATE` column will now try and tell you about the
    `chrono` feature. [[@abonander]]

    ```
    optional feature `chrono` required for type DATE of column #1 ("now")
    ```

[#62]: https://github.com/launchbadge/sqlx/issues/62
[#130]: https://github.com/launchbadge/sqlx/issues/130
[#98]: https://github.com/launchbadge/sqlx/pull/98
[#97]: https://github.com/launchbadge/sqlx/pull/97
[#134]: https://github.com/launchbadge/sqlx/pull/134
[#129]: https://github.com/launchbadge/sqlx/pull/129
[#131]: https://github.com/launchbadge/sqlx/pull/131
[#135]: https://github.com/launchbadge/sqlx/pull/135
[#108]: https://github.com/launchbadge/sqlx/pull/108

## 0.2.6 - 2020-03-10

### Added

-   [[#114]] Export `sqlx_core::Transaction` [[@thedodd]]

### Fixed

-   [[#125]] [[#126]] Fix statement execution in MySQL if it contains NULL statement values [[@repnop]]

-   [[#105]] [[#109]] Allow trailing commas in query macros [[@timmythetiny]]

[#105]: https://github.com/launchbadge/sqlx/pull/105
[#109]: https://github.com/launchbadge/sqlx/pull/109
[#114]: https://github.com/launchbadge/sqlx/pull/114
[#125]: https://github.com/launchbadge/sqlx/pull/125
[#126]: https://github.com/launchbadge/sqlx/pull/126
[@timmythetiny]: https://github.com/timmythetiny
[@thedodd]: https://github.com/thedodd

## 0.2.5 - 2020-02-01

### Fixed

-   Fix decoding of Rows containing NULLs in Postgres [#104]

-   After a large review and some battle testing by [@ianthetechie](https://github.com/ianthetechie)
    of the `Pool`, a live leaking issue was found. This has now been fixed by [@abonander] in [#84] which
    included refactoring to make the pool internals less brittle (using RAII instead of manual
    work is one example) and to help any future contributors when changing the pool internals.

-   Passwords are now being percent-decoded before being presented to the server [[@repnop]]

-   [@100] Fix `FLOAT` and `DOUBLE` decoding in MySQL

[#84]: https://github.com/launchbadge/sqlx/issues/84
[#100]: https://github.com/launchbadge/sqlx/issues/100
[#104]: https://github.com/launchbadge/sqlx/issues/104

### Added

-   [[#72]] Add `PgTypeInfo::with_oid` to allow simple construction of `PgTypeInfo` which enables `HasSqlType`
    to be implemented by downstream consumers of SQLx [[@jplatte]]

-   [[#96]] Add support for returning columns from `query!` with a name of a rust keyword by
    using raw identifiers [[@yaahc]]

-   [[#71]] Implement derives for `Encode` and `Decode`. This is the first step to supporting custom types in SQLx. [[@Freax13]]

[#72]: https://github.com/launchbadge/sqlx/issues/72
[#96]: https://github.com/launchbadge/sqlx/issues/96
[#71]: https://github.com/launchbadge/sqlx/issues/71

## 0.2.4 - 2020-01-18

### Fixed

-   Fix decoding of Rows containing NULLs in MySQL (and add an integration test so this doesn't break again)

## 0.2.3 - 2020-01-18

### Fixed

-   Fix `query!` when used on a query that does not return results

## 0.2.2 - 2020-01-16

### Added

-   [[#57]] Add support for unsigned integers and binary types in `query!` for MySQL [[@mehcode]]

[#57]: https://github.com/launchbadge/sqlx/issues/57

### Fixed

-   Fix stall when requesting TLS from a Postgres server that explicitly does not support TLS (such as postgres running inside docker) [[@abonander]]

-   [[#66]] Declare used features for `tokio` in `sqlx-macros` explicitly

[#66]: https://github.com/launchbadge/sqlx/issues/66

## 0.2.1 - 2020-01-16

### Fixed

-   [[#64], [#65]] Fix decoding of Rows containing NULLs in MySQL [[@danielakhterov]]

[#64]: https://github.com/launchbadge/sqlx/pull/64
[#65]: https://github.com/launchbadge/sqlx/pull/65

-   [[#55]] Use a shared tokio runtime for the `query!` macro compile-time execution (under the `runtime-tokio` feature) [[@udoprog]]

[#55]: https://github.com/launchbadge/sqlx/pull/55

## 0.2.0 - 2020-01-15

### Fixed

-   https://github.com/launchbadge/sqlx/issues/47

### Added

-   Support Tokio through an optional `runtime-tokio` feature.

-   Support SQL transactions. You may now use the `begin()` function on `Pool` or `Connection` to
    start a new SQL transaction. This returns `sqlx::Transaction` which will `ROLLBACK` on `Drop`
    or can be explicitly `COMMIT` using `commit()`.

-   Support TLS connections.

## 0.1.4 - 2020-01-11

### Fixed

-   https://github.com/launchbadge/sqlx/issues/43

-   https://github.com/launchbadge/sqlx/issues/40

### Added

-   Support for `SCRAM-SHA-256` authentication in Postgres [#37](https://github.com/launchbadge/sqlx/pull/37) [@danielakhterov](https://github.com/danielakhterov)

-   Implement `Debug` for Pool [#42](https://github.com/launchbadge/sqlx/pull/42) [@prettynatty](https://github.com/prettynatty)

## 0.1.3 - 2020-01-06

### Fixed

-   https://github.com/launchbadge/sqlx/issues/30

## 0.1.2 - 2020-01-03

### Added

-   Support for Authentication in MySQL 5+ including the newer authentication schemes now default in MySQL 8: `mysql_native_password`, `sha256_password`, and `caching_sha2_password`.

-   [`Chrono`](https://github.com/chronotope/chrono) support for MySQL was only partially implemented (was missing `NaiveTime` and `DateTime<Utc>`).

-   `Vec<u8>` (and `[u8]`) support for MySQL (`BLOB`) and Postgres (`BYTEA`).

[@abonander]: https://github.com/abonander
[@danielakhterov]: https://github.com/danielakhterov
[@mehcode]: https://github.com/mehcode
[@udoprog]: https://github.com/udoprog
[@jplatte]: https://github.com/jplatte
[@yaahc]: https://github.com/yaahc
[@freax13]: https://github.com/Freax13
[@repnop]: https://github.com/repnop
[@bmisiak]: https://github.com/bmisiak
[@oeb25]: https://github.com/oeb25
[@poiscript]: https://github.com/PoiScript
[@utter-step]: https://github.com/utter-step
[@sidred]: https://github.com/sidred
[@ace4896]: https://github.com/Ace4896
[@jamwaffles]: https://github.com/jamwaffles
[@nrjais]: https://github.com/nrjais
[@qtbeee]: https://github.com/qtbeee
[@xiaopengli89]: https://github.com/xiaopengli89
[@meh]: https://github.com/meh
[@shssoichiro]: https://github.com/shssoichiro
[@nilix007]: https://github.com/Nilix007
[@g-s-k]: https://github.com/g-s-k
[@blackwolf12333]: https://github.com/blackwolf12333
[@xyzd]: https://github.com/xyzd
[@hasali19]: https://github.com/hasali19
[@oriolmunoz]: https://github.com/OriolMunoz
[@pimeys]: https://github.com/pimeys
[@agentsim]: https://github.com/agentsim
[@meteficha]: https://github.com/meteficha
[@felipesere]: https://github.com/felipesere
[@dimtion]: https://github.com/dimtion
[@fundon]: https://github.com/fundon
[@aldaronlau]: https://github.com/AldaronLau
[@andrewwhitehead]: https://github.com/andrewwhitehead
[@slumber]: https://github.com/slumber
[@mcronce]: https://github.com/mcronce
[@hamza1311]: https://github.com/hamza1311
[@augustocdias]: https://github.com/augustocdias
[@pleto]: https://github.com/Pleto
[@chertov]: https://github.com/chertov
[@framp]: https://github.com/framp
[@markazmierczak]: https://github.com/markazmierczak
[@msrd0]: https://github.com/msrd0
[@joshtriplett]: https://github.com/joshtriplett
[@nyxcode]: https://github.com/NyxCode
[@nitsky]: https://github.com/nitsky
[@esemeniuc]: https://github.com/esemeniuc
[@iamsiddhant05]: https://github.com/iamsiddhant05
[@dstoeckel]: https://github.com/dstoeckel
[@mrcd]: https://github.com/mrcd
[@dvermd]: https://github.com/dvermd
[@seryl]: https://github.com/seryl
[@ant32]: https://github.com/ant32
[@robjtede]: https://github.com/robjtede
[@pymongo]: https://github.com/pymongo
[@sile]: https://github.com/sile
[@fl9]: https://github.com/fl9
[@antialize]: https://github.com/antialize
[@dignifiedquire]: https://github.com/dignifiedquire
[@argv-minus-one]: https://github.com/argv-minus-one
[@qqwa]: https://github.com/qqwa
[@diggsey]: https://github.com/Diggsey
[@crajcan]: https://github.com/crajcan
[@demurgos]: https://github.com/demurgos
[@link2xt]: https://github.com/link2xt
[@guylapid]: https://github.com/guylapid
[@natproach]: https://github.com/NatPRoach
[@feikesteenbergen]: https://github.com/feikesteenbergen
[@etcaton]: https://github.com/ETCaton
[@toshokan]: https://github.com/toshokan
[@nomick]: https://github.com/nomick
[@marshoepial]: https://github.com/marshoepial
[@link2ext]: https://github.com/link2ext
[@madadam]: https://github.com/madadam
[@AtkinsChang]: https://github.com/AtkinsChang
[@djmarcin]: https://github.com/djmarcin
[@ghassmo]: https://github.com/ghassmo
[@eagletmt]: https://github.com/eagletmt
[@montanalow]: https://github.com/montanalow
[@nitnelave]: https://github.com/nitnelave
[@Drevoed]: https://github.com/Drevoed
[@yuyawk]: https://github.com/yuyawk
[@yerke]: https://github.com/yerke
[@russweas]: https://github.com/russweas
[@zbigniewzolnierowicz]: https://github.com/zbigniewzolnierowicz
[@dimfeld]: https://github.com/dimfeld
[@akiradeveloper]: https://github.com/akiradeveloper
[@chesedo]: https://github.com/chesedo
[@LLBlumire]: https://github.com/LLBlumire
[@liushuyu]: https://github.com/liushuyu
[@paolobarbolini]: https://github.com/paolobarbolini
[@DoumanAsh]: https://github.com/DoumanAsh
[@D1plo1d]: https://github.com/D1plo1d
[@tkintscher]: https://github.com/tkintscher
[@SonicZentropy]: https://github.com/SonicZentropy
[@parazyd]: https://github.com/parazyd
[@kunjee17]: https://github.com/kunjee17
[@05storm26]: https://github.com/05storm26
[@dbeckwith]: https://github.com/dbeckwith
[@k-jun]: https://github.com/k-jun
[@tranzystorek-io]: https://github.com/tranzystorek-io
[@taladar]: https://github.com/taladar
[@genusistimelord]: https://github.com/genusistimelord
[@p9s]: https://github.com/p9s
[@ArGGu]: https://github.com/ArGGu
[@sedrik]: https://github.com/sedrik
[@nappa85]: https://github.com/nappa85
[@ifn3]: https://github.com/ifn3
[@LovecraftianHorror]: https://github.com/LovecraftianHorror
[@stoically]: https://github.com/stoically
[@VersBinarii]: https://github.com/VersBinarii
[@cemoktra]: https://github.com/cemoktra
[@jdrouet]: https://github.com/jdrouet
[@vbmade2000]: https://github.com/vbmade2000
[@abreis]: https://github.com/abreis
[@0xdeafbeef]: https://github.com/0xdeafbeef
[@Dylan-DPC]: https://github.com/Dylan-DPC
[@carols10cents]: https://github.com/carols10cents
[@david-mcgillicuddy-moixa]: https://github.com/david-mcgillicuddy-moixa
[@ipetkov]: https://github.com/ipetkov
[@pedromfedricci]: https://github.com/pedromfedricci
[@tm-drtina]: https://github.com/tm-drtina
[@espindola]: https://github.com/espindola
[@mgrachev]: https://github.com/mgrachev
[@tyrelr]: https://github.com/tyrelr
[@SebastienGllmt]: https://github.com/SebastienGllmt
[@e00E]: https://github.com/e00E
[@sebpuetz]: https://github.com/sebpuetz
[@pruthvikar]: https://github.com/pruthvikar
[@tobymurray]: https://github.com/tobymurray
[@djc]: https://github.com/djc
[@mfreeborn]: https://github.com/mfreeborn
[@scottwey]: https://github.com/scottwey
[@e-rhodes]: https://github.com/e-rhodes
[@OskarPersson]: https://github.com/OskarPersson
[@walf443]: https://github.com/walf443
[@lovasoa]: https://github.com/lovasoa
[@mdtusz]: https://github.com/mdtusz
[@kianmeng]: https://github.com/kianmeng
[@EthanYuan]: https://github.com/EthanYuan
[@Nukesor]: https://github.com/Nukesor
[@smonv]: https://github.com/smonv
[@Erik1000]: https://github.com/Erik1000
[@raviqqe]: https://github.com/raviqqe
[@johnbcodes]: https://github.com/johnbcodes
[@sbeckeriv]: https://github.com/sbeckeriv
[@RomainStorai]: https://github.com/RomainStorai
[@jayy-lmao]: https://github.com/jayy-lmao
[@Thomasdezeeuw]: https://github.com/Thomasdezeeuw
[@kenkoooo]: https://github.com/kenkoooo
[@TheoOiry]: https://github.com/TheoOiry
[@JoeyMckenzie]: https://github.com/JoeyMckenzie
[@ivan]: https://github.com/ivan
[@crepererum]: https://github.com/crepererum
[@UramnOIL]: https://github.com/UramnOIL
[@liningpan]: https://github.com/liningpan
[@zzhengzhuo]: https://github.com/zzhengzhuo
[@crepererum]: https://github.com/crepererum
[@szymek156]: https://github.com/szymek156
[@NSMustache]: https://github.com/NSMustache
[@RustyYato]: https://github.com/RustyYato
[@alexander-jackson]: https://github.com/alexander-jackson
[@zlidner]: https://github.com/zlidner
[@zlindner]: https://github.com/zlindner
[@marcustut]: https://github.com/marcustut
[@rakshith-ravi]: https://github.com/rakshith-ravi
[@bradfier]: https://github.com/bradfier
[@fuzzbuck]: https://github.com/fuzzbuck
[@cycraig]: https://github.com/cycraig
[@fasterthanlime]: https://github.com/fasterthanlime
[@he4d]: https://github.com/he4d
[@DXist]: https://github.com/DXist
[@Wopple]: https://github.com/Wopple
[@TravisWhitehead]: https://github.com/TravisWhitehead
[@ThibsG]: https://github.com/ThibsG
[@rongcuid]: https://github.com/rongcuid
[@moatra]: https://github.com/moatra
[@penberg]: https://github.com/penberg
[@saiintbrisson]: https://github.com/saiintbrisson
[@FSMaxB]: https://github.com/FSMaxB
[@95ulisse]: https://github.com/95ulisse
[@miles170]: https://github.com/miles170
[@ar3s3ru]: https://github.com/ar3s3ru
[@cdbfoster]: https://github.com/cdbfoster
[@andyquinterom]: https://github.com/andyquinterom
[@CosmicHorrorDev]: https://github.com/CosmicHorrorDev
[@VictorKoenders]: https://github.com/VictorKoenders
[@joehillen]: https://github.com/joehillen
[@OverHash]: https://github.com/OverHash
[@laundmo]: https://github.com/laundmo
[@nbaztec]: https://github.com/nbaztec
[@bgeron]: https://github.com/bgeron
[@benesch]: https://github.com/benesch
[@nstinus]: https://github.com/nstinus
[@grgi]: https://github.com/grgi
[@sergeiivankov]: https://github.com/sergeiivankov
[@jaysonsantos]: https://github.com/jaysonsantos
[@dbrgn]: https://github.com/dbrgn
[@grantkee]: https://github.com/grantkee
[@bnoctis]: https://github.com/bnoctis
[@aschey]: https://github.com/aschey
[@df51d]: https://github.com/df51d
[@codahale]: https://github.com/codahale
[@arlyon]: https://github.com/arlyon
[@SergioBenitez]: https://github.com/SergioBenitez
[@tgeoghegan]: https://github.com/tgeoghegan
[@vizvasrj]: https://github.com/vizvasrj
[@phlip9]: https://github.com/phlip9
[@MidasLamb]: https://github.com/MidasLamb
[@utkarshgupta137]: https://github.com/utkarshgupta137
[@Arcayr]: https://github.com/Arcayr
[@mdecimus]: https://github.com/mdecimus
[@Razican]: https://github.com/Razican
[@southball]: https://github.com/southball
[@alilleybrinker]: https://github.com/alilleybrinker
[@titaniumtraveler]: https://github.com/titaniumtraveler
[@nyurik]: https://github.com/nyurik
[@stepantubanov]: https://github.com/stepantubanov
[@iamquang95]: https://github.com/iamquang95
[@jnnnnn]: https://github.com/jnnnnn
[@saolof]: https://github.com/saolof
[@deneut]: https://github.com/deneut
[@kitterion]: https://github.com/kitterion
[@denschub]: https://github.com/denschub
[@fd]: https://github.com/fd
[@mrl5]: https://github.com/mrl5
[@Xydez]: https://github.com/Xydez
[@vabka]: https://github.com/vabka
[@ldanilek]: https://github.com/ldanilek
[@inahga]: https://github.com/inahga
[@JockeM]: https://github.com/JockeM
[@vmax]: https://github.com/vmax
[@sebastianv89]: https://github.com/sebastianv89
[@marcusirgens]: https://github.com/marcusirgens
[@tsing]: https://github.com/tsing
[@snf]: https://github.com/snf
[@wyhaya]: https://github.com/wyhaya
[@hakoerber]: https://github.com/hakoerber
[@olback]: https://github.com/olback
[@kryptan]: https://github.com/kryptan
[@grant0417]: https://github.com/grant0417
[@fermanjj]: https://github.com/fermanjj
[@grooverdan]: https://github.com/grooverdan
[@bobozaur]: https://github.com/bobozaur
[@aldur]: https://github.com/aldur
[@hgranthorner]: https://github.com/hgranthorner
[@ripa1995]: https://github.com/ripa1995
[@fhsgoncalves]: https://github.com/fhsgoncalves
[@uttarayan21]: https://github.com/uttarayan21
[@tk2217]: https://github.com/tk2217
[@hamiltop]: https://github.com/hamiltop
[@cameronbraid]: https://github.com/cameronbraid
[@Baptistemontan]: https://github.com/Baptistemontan
[@satwanjyu]: https://github.com/satwanjyu
[@boris-lok]: https://github.com/boris-lok
[@qwerty2501]: https://github.com/qwerty2501
[@Nemo157]: https://github.com/Nemo157
[@snspinn]: https://github.com/snspinn
[@granddaifuku]: https://github.com/granddaifuku
[@yasamoka]: https://github.com/yasamoka
[@mattfbacon]: https://github.com/mattfbacon
[@A248]: https://github.com/A248
[@conradludgate]: https://github.com/conradludgate
[@anupj]: https://github.com/anupj
[@nanoqsh]: https://github.com/nanoqsh
[@brianheineman]: https://github.com/brianheineman
[@jkleinknox]: https://github.com/jkleinknox
[@cryeprecision]: https://github.com/cryeprecision
[@Vrajs16]: https://github.com/Vrajs16
[@shiftrightonce]: https://github.com/shiftrightonce
[@tamasfe]: https://github.com/tamasfe
[@lily-mosquitoes]: https://github.com/lily-mosquitoes
[@larsschumacher]: https://github.com/larsschumacher
[@shengsheng]: https://github.com/shengsheng
[@Fyko]: https://github.com/Fyko
[@kshramt]: https://github.com/kshramt
[@Dawsoncodes]: https://github.com/Dawsoncodes
[@tadghh]: https://github.com/tadghh
[@holicc]: https://github.com/holicc
[@takenoko-gohan]: https://github.com/takenoko-gohan
[@iangilfillan]: https://github.com/iangilfillan
[@iamjpotts]: https://github.com/iamjpotts
[@Icerath]: https://github.com/Icerath
[@pawurb]: https://github.com/pawurb
[@darkecho731]: https://github.com/darkecho731
[@mirek26]: https://github.com/mirek26
[@Ekleog]: https://github.com/Ekleog
[@zoomiti]: https://github.com/zoomiti
[@ciffelia]: https://github.com/ciffelia
[@rafaelGuerreiro]: https://github.com/rafaelGuerreiro
[@alu]: https://github.com/alu
[@BadBastion]: https://github.com/BadBastion
[@tylerhawkes]: https://github.com/tylerhawkes
[@g-bartoszek]: https://github.com/g-bartoszek
[@benluelo]: https://github.com/benluelo
[@ralpha]: https://github.com/ralpha
[@nitn3lav]: https://github.com/nitn3lav
[@FlakM]: https://github.com/FlakM
[@hoxxep]: https://github.com/hoxxep
[@NfNitLoop]: https://github.com/NfNitLoop
[@GnomedDev]: https://github.com/GnomedDev
[@pxp9]: https://github.com/pxp9
[@RaghavRox]: https://github.com/RaghavRox
[@cleverjam]: https://github.com/cleverjam
[@BlackSoulHub]: https://github.com/BlackSoulHub
[@levkk]: https://github.com/levkk
[@danjpgriffin]: https://github.com/danjpgriffin
[@toxeus]: https://github.com/toxeus
[@jasonish]: https://github.com/jasonish
[@AlphaKeks]: https://github.com/AlphaKeks
[@Zarathustra2]: https://github.com/Zarathustra2
[@gridbox]: https://github.com/gridbox
[@joelkoen]: https://github.com/joelkoen
[@nk9]: https://github.com/nk9
[@etorreborre]: https://github.com/etorreborre
[@LecrisUT]: https://github.com/LecrisUT
[@JohannesIBK]: https://github.com/JohannesIBK
[@Lachstec]: https://github.com/Lachstec
[@SrGesus]: https://github.com/SrGesus
[@CommanderStorm]: https://github.com/CommanderStorm
[@hamirmahal]: https://github.com/hamirmahal
[@DirectorX]: https://github.com/DirectorX
[@KobusEllis]: https://github.com/KobusEllis
[@YgorSouza]: https://github.com/YgorSouza
[@Zarthus]: https://github.com/Zarthus
[@ckampfe]: https://github.com/ckampfe
[@tottoto]: https://github.com/tottoto
[@ods]: https://github.com/ods
[@soucosmo]: https://github.com/soucosmo
[@kolinfluence]: https://github.com/kolinfluence
[@joeydewaal]: https://github.com/joeydewaal
[@pierre-wehbe]: https://github.com/pierre-wehbe
[@carschandler]: https://github.com/carschandler
[@kdesjard]: https://github.com/kdesjard
[@luveti]: https://github.com/luveti
[@dojiong]: https://github.com/dojiong
[@jayvdb]: https://github.com/jayvdb
[@kurtbuilds]: https://github.com/kurtbuilds
[@lilydjwg]: https://github.com/lilydjwg
[@M3t0r]: https://github.com/M3t0r
[@vsuryamurthy]: https://github.com/vsuryamurthy
[@manifest]: https://github.com/manifest
[@tbar4]: https://github.com/tbar4
[@sandhose]: https://github.com/sandhose
[@IlyaBizyaev]: https://github.com/IlyaBizyaev
[@philipcristiano]: https://github.com/philipcristiano
[@xuehaonan27]: https://github.com/xuehaonan27
[@chanks]: https://github.com/chanks
[@Ddystopia]: https://github.com/Ddystopia
[@veigaribo]: https://github.com/veigaribo
[@Norlock]: https://github.com/Norlock
[@swlynch99]: https://github.com/swlynch99
[@BenoitRanque]: https://github.com/BenoitRanque
[@hsivonen]: https://github.com/hsivonen
[@andreweggleston]: https://github.com/andreweggleston
[@Suficio]: https://github.com/Suficio
[@bonega]: https://github.com/bonega
[@nico-incubiq]: https://github.com/nico-incubiq
[@tisonkun]: https://github.com/tisonkun
[@karambarakat]: https://github.com/karambarakat
[@seanaye]: https://github.com/seanaye
[@remysaissy]: https://github.com/remysaissy
[@BeauGieskens]: https://github.com/BeauGieskens
[@Turbo87]: https://github.com/Turbo87
[@jthacker]: https://github.com/jthacker
[@benwilber]: https://github.com/benwilber
[@chitoku-k]: https://github.com/chitoku-k
[@chanmaoganda]: https://github.com/chanmaoganda
[@dns2utf8]: https://github.com/dns2utf8
[@mattrighetti]: https://github.com/mattrighetti
[@soulwa]: https://github.com/soulwa
[@kildrens]: https://github.com/kildrens
[@xvapx]: https://github.com/xvapx
[@jonasmalacofilho]: https://github.com/jonasmalacofilho
[@sulami]: https://github.com/sulami
[@thriller08]: https://github.com/thriller08
[@mbj]: https://github.com/mbj
[@TeCHiScy]: https://github.com/TeCHiScy
[@mpyw]: https://github.com/mpyw
[@bonsairobo]: https://github.com/bonsairobo
[@gferon]: https://github.com/gferon
[@joshka]: https://github.com/joshka
[@kujeger]: https://github.com/kujeger
[@dyc3]: https://github.com/dyc3
[@ThomWright]: https://github.com/ThomWright
[@duhby]: https://github.com/duhby
