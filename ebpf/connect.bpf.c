#include "vmlinux.h"

#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_endian.h>

char LICENSE[] SEC("license") = "GPL";

struct conn_event {
    u32 pid;

    u32 saddr;
    u32 daddr;

    u16 sport;
    u16 dport;

    u16 family;
    u16 protocol;

    u32 oldstate;
    u32 newstate;

    char comm[16];
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 24);
} events SEC(".maps");

/*
 * Tracepoint layout from:
 * /sys/kernel/debug/tracing/events/sock/inet_sock_set_state/format
 */

SEC("tracepoint/sock/inet_sock_set_state")
int handle_state(struct trace_event_raw_inet_sock_set_state *ctx)
{
    struct conn_event *e;

    e = bpf_ringbuf_reserve(&events, sizeof(*e), 0);
    if (!e)
        return 0;

    e->pid = bpf_get_current_pid_tgid() >> 32;

    bpf_get_current_comm(&e->comm, sizeof(e->comm));

    __builtin_memcpy(&e->saddr, ctx->saddr, 4);
    __builtin_memcpy(&e->daddr, ctx->daddr, 4);

    e->sport = ctx->sport;
    e->dport = ctx->dport;

    e->family = ctx->family;
    e->protocol = ctx->protocol;

    e->oldstate = ctx->oldstate;
    e->newstate = ctx->newstate;

    /* AF_INET only for now */
    if (e->family != 2)
        goto out;

    /* ESTABLISHED only */
    if (e->newstate != 1) {
        bpf_ringbuf_discard(e, 0);
    return 0;}

out:
    bpf_ringbuf_submit(e, 0);
    return 0;
}
