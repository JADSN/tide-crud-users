# BrickPack SampleApp

A sample CNA (Cloud Native Application)/Microservice writen using brickpack: https://github.com/afsec/brickpack

## Lean artifact (< 10 MB)
The whole microservice is built with static compiling using **MUSL** target.
```
$ ls -lh
total 2,3M
-rwxrwxr-x 1 user user 2,3M nov 30 20:15 brickpack-sampleapp
```

## Startup message
```
$ make release run
./scripts/build.sh
   Compiling brickpack v0.9.5 (/home/user/src/brickpack/brickpack)
   Compiling brickpack-sampleapp v0.1.0 (/home/user/src/brickpack-sampleapp)
[brickpack_derive] Endpoint detected: (endpoint_name = add_user)
[brickpack_derive] Endpoint detected: (endpoint_name = del_user)
[brickpack_derive] Endpoint detected: (endpoint_name = show_users)
    Finished release [optimized] target(s) in 22.17s
'./target/x86_64-unknown-linux-musl/release/brickpack-sampleapp' -> './dist/brickpack-sampleapp'
./scripts/run.sh
{"level":30,"time":1606862446886,"msg":"Logger started","level":Info}
{"level":30,"time":1606862446886,"msg":"Starting App [brickpack-sampleapp v0.1.0]:"}
{"level":30,"time":1606862446886,"msg":"Powered by Brickpack Web Framework v0.9.5"}
{"level":30,"time":1606862446887,"msg":"Server listening on http://127.0.0.1:8080"}
```

---

[![Dependencies Status](https://deps.rs/repo/github/afsec/brickpack-sampleapp/status.svg)](https://deps.rs/repo/github/afsec/brickpack-sampleapp)
