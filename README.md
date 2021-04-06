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
$ LOG_LEVEL=DEBUG ./tide-crud-users
{"level":30,"time":1617734910475,"msg":"Logger started","level":DEBUG}
{"level":30,"time":1617734910475,"msg":"Starting App [tide-crud-users v0.4.0]:"}
{"level":30,"time":1617734910478,"msg":"Server listening on http://127.0.0.1:8080"}
```

## Show endpoints
```
$ ./tide-crud-users -e
  Internal Endpoints:
    /                - index_page
    /maintenance     - maintenance
    /auth            - check_auth
  
  Endpoints:
    /api/add_user - AddUser
    /api/delete_user - DeleteUser
    /api/export_users - ExportUsers
    /api/show_user - ShowUser
    /api/show_users - ShowUsers
    /api/update_user - UpdateUser
```

---

[![Dependencies Status](https://deps.rs/repo/github/afsec/tide_crud_users_sqlite/status.svg)](https://deps.rs/repo/github/afsec/tide_crud_users_sqlite)
