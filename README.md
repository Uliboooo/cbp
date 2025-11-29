# CodeBasePrinter

current version: 0.7.0

## build

```zsh
V=$(git describe --tags --always) && go build -ldflags="-X 'main.Version=${V}'" -o cbp
```

## TODO

? is a feature whose implementation is undecided.

- [ ] Use Code block \`\`\`
- [ ] Hide folder name without files in tree
- [ ] ~~Send OS clipboard?~~
    - The combination of pipe and pbcopy is sufficient.

## Usage

```sh
Usage of cbp:
  -filter-ext lua
        filter extensions. e.g. use lua
  -ignore-ext lua,json
        ignore extensions. e.g., lua,json
  -ignore-fld release,test
        Ignore folders. e.g., use release,test
  -no-tree
        hide tree.
  -token-limit 10000
        token limit. e.g. use 10000 (default -1)
  -tree only
        tree option (none, only). e.g. only
  -version
        show current version
```

```bash
cbp -ignore_ext "go,sum,md"
cbp/
│   ├─ .DS_Store
│   ├─ .gitignore
│   ├─ LICENSE
│   ├─ go.mod
    └─ release/
    └─ │   ├─ cbp
    └─ │   ├─ cbp_arm_mac.zip
    └─ │   ├─ v0-2-0/
    └─ │   ├─ │   ├─ .DS_Store
    └─ │   ├─     └─ cbp
    └─     └─ v0-3-0/
    └─     └─ │   ├─ cbp_mac_0_3_0
    └─     └─ │   ├─ cbp_mac_0_3_0.zip
    └─     └─ │   ├─ cbp_win_0_3_0.exe
    └─     └─     └─ cbp_win_0_3_0.zip
------------------------------------------------------------------------
~/Develop/cbp/.gitignore
------------------------------------------------------------------------
release/

------------------------------------------------------------------------
~/Develop/cbp/LICENSE
------------------------------------------------------------------------
This software is licensed under either:
- The MIT License
- OR The Apache License 2.0

## MIT

MIT License

Copyright (c) 2025 Uliboooo

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Apache-2.0

Copyright 2025 Uliboooo

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

------------------------------------------------------------------------
~/Develop/cbp/go.mod
------------------------------------------------------------------------
module example/cbp

go 1.25.4

require (
        golang.org/x/sys v0.38.0 // indirect
        golang.org/x/term v0.37.0 // indirect
)
```

