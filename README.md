# Tide CRUD Users

Just a Rust CRUD powered by rusSQLite, r2d2_sqlite and tide-rs.

## Lean artifact (< 10 MB)
The whole artifact is built with static compiling using **MUSL** target.
```
$ ls -lh
total 4,1M
-rwxrwxr-x 1 user user 4,1M abr  3 16:54 tide-crud-users
```

## Startup message
```
$ make release run
./scripts/build.sh
    Finished release [optimized] target(s) in 0.11s
'./target/x86_64-unknown-linux-musl/release/tide-crud-users' -> './dist/tide-crud-users'
./scripts/run.sh
{"level":30,"time":1617479671289,"msg":"Logger started","level":Info}
{"level":30,"time":1617479671290,"msg":"Starting App [tide-crud-users v0.3.10]:"}
{"level":30,"time":1617479671290,"msg":"Server listening on http://127.0.0.1:8080"}
```

---

[![Dependencies Status](https://deps.rs/repo/github/afsec/tide_crud_users_sqlite/status.svg)](https://deps.rs/repo/github/afsec/tide_crud_users_sqlite)
