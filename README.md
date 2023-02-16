# File_CLOAK

hidding file by eBPF program.

## Main Process in eBPF program

Hook getdents64 syscall enter: Store the function paramter linux_dirent64 *dirp.

Hook getdents64 Syscall exit: Find target directory by name(handle_getdents_exit). Unlink target by reading in previous linux_dirent64 struct and setting it's d_reclen to cover itself and our target(handle_getdents_patch). (So the target file actually need to have a previous file)

Credit to https://github.com/pathtofile/bad-bpf/

## Usage

```
cargo build --release
sudo target/release/file_cloak <file or directory>
```

## Test

ENABLE DEBUG MODE AT ``common.h:9``

```
cargo build
sudo target/debug/file_cloak thisisanother
```

your can watch the debug output

```
sudo cat  /sys/kernel/debug/tracing/trace_pip
```

```
> ls test
test.py  thisisatest
```

You can **Hide Process** via hiding corresponding pid directory in ``/proc``. XD

## vmlinux.h

this repo contains a vmlinux.h which is generated in Linux DUBHE-VM 5.15.0-58-generic #64-Ubuntu

To generate an updated `vmlinux.h`:

```shell
$ bpftool btf dump file /sys/kernel/btf/vmlinux format c > ./vmlinux.h
$ ln -s ./vmlinux.h src/bpf/vmlinux.h
```

## Disclaimer

Do not attempt to use these tools to violate the law. The author is not responsible for any illegal action. Misuse of the provided information can result in criminal charges.
