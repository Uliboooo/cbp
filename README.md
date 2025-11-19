# CodeBasePrinter

## test

```bash
❯ go run . -ignore_ext "go,sum,md"
cbp/
└─ go.mod
───────────────────────────────────────────────
~/Develop/cbp/go.mod
───────────────────────────────────────────────
&module example/cbp

go 1.25.4

require (
        golang.org/x/sys v0.38.0 // indirect
        golang.org/x/term v0.37.0 // indirect
)
```

