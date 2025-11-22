# CodeBasePrinter

current version: 0.2.0

## TODO

? is a feature whose implementation is undecided.

- [ ] Use Code block \`\`\`
- [ ] Hide folder name without files in tree
- [ ] Send OS clipboard?

## test

```bash
❯ ls
go.mod  go.sum  LICENSE  main.go  README.md


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

