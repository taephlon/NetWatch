# Building and Running NetWatch eBPF Programs

## Prerequisites

Before compiling the eBPF program, ensure the following packages are installed:

### Gentoo

```bash
sudo emerge -av \
    dev-lang/rust \
    sys-devel/clang \
    sys-devel/llvm \
    dev-libs/libbpf \
    sys-apps/bpftool \
    dev-util/pahole \
    dev-db/sqlite
```

Verify the installation:

```bash
rustc --version
cargo --version
clang --version
bpftool version
```

---

## Generate `vmlinux.h`

NetWatch uses CO-RE (Compile Once – Run Everywhere), which requires a generated `vmlinux.h` file from the running kernel's BTF information.

Generate it with:

```bash
bpftool btf dump file /sys/kernel/btf/vmlinux format c > ebpf/vmlinux.h
```

You should now have:

```text
ebpf/
├── connect.bpf.c
├── vmlinux.h
```

---

## Compile the eBPF Program

From the project root directory:

```bash
clang \
-target bpf \
-g \
-O2 \
-D__TARGET_ARCH_x86 \
-I./ebpf \
-c ebpf/connect.bpf.c \
-o ebpf/connect.bpf.o
```

This produces:

```text
ebpf/connect.bpf.o
```

which is the eBPF object loaded by NetWatch.

---

## Verify the Generated Object

Check that the file was compiled for the BPF target:

```bash
file ebpf/connect.bpf.o
```

Expected output:

```text
ELF 64-bit LSB relocatable, eBPF
```

You can also inspect the ELF sections:

```bash
llvm-objdump -h ebpf/connect.bpf.o
```

Expected sections include:

```text
maps
license
tracepoint/*
```

or

```text
tp_btf/*
```

depending on the implementation.

---

## Build the Rust Userspace Application

Compile NetWatch:

```bash
cargo build
```

For an optimized release build:

```bash
cargo build --release
```

---

## Run NetWatch

Launch the application:

```bash
cargo run
```

or

```bash
./target/release/netwatch
```

Expected output:

```text
Loaded 2 threat indicators
NetWatch running on :3000
```

---

## Open the Dashboard

Open your browser and navigate to:

```text
http://localhost:3000
```

The dashboard should display:

* Live TCP connection events
* Process names
* Executable paths
* Reverse DNS hostnames
* GeoIP information
* Threat scores
* Threat labels
* WebSocket-powered real-time updates

---

## Troubleshooting

### BPF Target Not Found

Verify LLVM supports BPF:

```bash
llc --version | grep BPF
```

Expected:

```text
bpf
bpfel
bpfeb
```

---

### `events map not found`

Ensure the eBPF program defines a ring buffer map named:

```c
events
```

and that `connect.bpf.o` was rebuilt after changes.

---

### `failed to open object file`

Verify the object exists:

```bash
ls -lah ebpf/connect.bpf.o
```

If not, recompile the eBPF program.

---

### Web Dashboard Shows No Events

Check:

1. The WebSocket endpoint is reachable.
2. The eBPF object loaded successfully.
3. The ring buffer callback is receiving events.
4. Browser developer tools show no JavaScript errors.

---

## Project Workflow

```text
connect.bpf.c
        │
        ▼
clang -target bpf
        │
        ▼
connect.bpf.o
        │
        ▼
libbpf-rs Loader
        │
        ▼
Ring Buffer
        │
        ▼
Rust Backend
        │
        ▼
SQLite Database
        │
        ▼
WebSocket API
        │
        ▼
NetWatch Dashboard
```
