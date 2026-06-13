## Migrating from Binary

If you're used to binary percolation (flow / no flow), ternary adds a **critical** state — the $0$ at the transition threshold.

| Binary | Ternary |
|--------|---------|
| Flow ($1$) | Percolated ($+1$) |
| No flow ($0$) | Critical ($0$) |
| | Blocked ($-1$) |

Binary percolation models flip instantly from blocking to flowing at the threshold. Ternary captures the critical zone where clusters are forming but haven't connected yet — the most interesting dynamics happen here.

See **[From Binary to Ternary](https://github.com/SuperInstance/ternary-cookbook/blob/master/guides/FROM_BINARY.md)** for the full migration guide.
