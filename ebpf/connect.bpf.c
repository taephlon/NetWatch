#include "vmlinux.h"

#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "GPL";

struct conn_event {
    __u32 pid;

    __u32 saddr;
    __u32 daddr;

    __u16 sport;
    __u16 dport;

    __u16 family;
    __u16 protocol;

    __u32 oldstate;
    __u32 newstate;

    char comm[16];
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 24);
} events SEC(".maps");

SEC("tracepoint/sock/inet_sock_set_state")
int trace_inet_sock_set_state(
    struct trace_event_raw_inet_sock_set_state *ctx
)
{
    struct conn_event *e;

    e = bpf_ringbuf_reserve(
            &events,
            sizeof(*e),
            0
        );

    if (!e)
        return 0;

    e->pid =
        bpf_get_current_pid_tgid() >> 32;

    bpf_get_current_comm(
        &e->comm,
        sizeof(e->comm)
    );

    __builtin_memcpy(&e->saddr, ctx->saddr, sizeof(__u32));
    __builtin_memcpy(&e->daddr, ctx->daddr, sizeof(__u32));

    e->sport = ctx->sport;
    e->dport = ctx->dport;

    e->family = ctx->family;

    e->oldstate = ctx->oldstate;
    e->newstate = ctx->newstate;

    bpf_ringbuf_submit(e, 0);

    return 0;
}
