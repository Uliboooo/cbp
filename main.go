package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"os/user"
	"path/filepath"
	"slices"
	"strings"

	"github.com/pkoukk/tiktoken-go"
	"golang.org/x/term"
)

type FileNode struct {
	Name     string
	Path     string
	IsDir    bool
	Children []*FileNode
}

func userHome() (string, error) {
	currentUser, err := user.Current()
	return currentUser.HomeDir, err
}

func isIncludeFdr(str string, lst []string) bool {
	return slices.Contains(lst, str)
}

func isIncludeEx(str string, lst []string) bool {
	ext := filepath.Ext(str)
	if ext == "" {
		return false
	}

	res := slices.Contains(lst, strings.Trim(ext, "."))
	return res
}

func buildTree(path string, ignoreExt []string, ignoreFdr []string, filtExt []string) (*FileNode, error) {
	node := &FileNode{
		Name:  filepath.Base(path),
		Path:  path,
		IsDir: true,
	}

	info, err := os.Stat(path)
	if err != nil {
		return nil, err
	}
	node.IsDir = info.IsDir()

	if !node.IsDir {
		if isIncludeEx(node.Name, ignoreExt) {
			return nil, nil
		}
		if len(filtExt) > 0 && !isIncludeEx(node.Name, filtExt) {
			return nil, nil
		}
		// node.Name = filepath.Base(path)
		return node, nil
	}

	if isIncludeFdr(node.Name, ignoreFdr) {
		return nil, nil
	}

	entries, err := os.ReadDir(path)
	if err != nil {
		return nil, err
	}

	for _, e := range entries {
		if e.Name() == "." || e.Name() == ".." {
			continue
		}
		chilePath := filepath.Join(path, e.Name())
		childPath, err := buildTree(chilePath, ignoreExt, ignoreFdr, filtExt)
		if err != nil {
			log.Printf("errr in process child node: %v", err)
			continue
		}
		if childPath != nil {
			node.Children = append(node.Children, childPath)
		}
	}

	return node, nil
}

func formatTree(node *FileNode) string {
	var builder strings.Builder
	formatTreeRec(node, "", &builder)
	return builder.String()

}

func formatTreeRec(node *FileNode, prefix string, builder *strings.Builder) {
	if node == nil {
		return
	}
	name := node.Name
	if node.IsDir {
		name = fmt.Sprintf("\033[34m%s/\033[0m", name)
	}

	builder.WriteString(prefix)
	builder.WriteString(name)
	builder.WriteString("\n")

	for i, child := range node.Children {
		isLast := i == len(node.Children)-1

		var newPrefix string
		if isLast {
			newPrefix = prefix + "  "
		} else {
			newPrefix = prefix + "│ "
		}

		currentPrefix := "│─"
		if isLast {
			currentPrefix = "└─"
		}

		formatTreeRec(child, newPrefix+currentPrefix, builder)
	}
}

func mulStr(str string, mul int) string {
	var s string
	for range mul {
		s = s + str

	}
	return s
}

func PrivateRemover(path string) string {
	h, e := userHome()
	if e != nil {
		h = ""
	}
	res := strings.ReplaceAll(path, h, "~")
	return res
}

func formatFiles(node *FileNode) string {
	var builder strings.Builder
	formatFilesRec(node, &builder)
	return builder.String()
}

func formatFilesRec(node *FileNode, builder *strings.Builder) {
	if node == nil {
		return
	}

	if !node.IsDir {
		var x int
		fd := int(os.Stdout.Fd())
		if !term.IsTerminal(fd) {
			x = 20
		} else {
			width, _, _ := term.GetSize(fd)
			x = width / 2
		}

		cont, err := os.ReadFile(node.Path)
		if err != nil {
			return
		}

		if isBin(cont) {
			return
		}

		line := mulStr("-", x)
		fmt.Fprintf(builder, "%s\n%s\n%s\n%s\n", line, PrivateRemover(node.Path), line, cont)
		return
	}

	for _, child := range node.Children {
		formatFilesRec(child, builder)
	}

}

func isBin(content []byte) bool {
	checkLen := min(len(content), 1024)

	return slices.Contains(content[:checkLen], 0)
}

func truncateTokens(s string, limit int) (*string, error) {
	tkm, err := tiktoken.EncodingForModel("gpt-4")
	if err != nil {
		return nil, err
	}

	token := tkm.Encode(s, nil, nil)

	if len(token) <= limit {
		return &s, nil
	}
	truncatedToekns := token[:limit]
	res := tkm.Decode(truncatedToekns)
	return &res, nil
}

func main() {
	igExtPtr := flag.String("ignore-ext", "", "ignore extensions")
	igFldPtr := flag.String("ignore-fld", "", "ignore folders")
	filtExPtr := flag.String("filter-ext", "", "filter extensions")
	tokenLimit := flag.Int("token-limit", -1, "token limit")

	flag.Parse()

	igExt := strings.Split(*igExtPtr, ",")
	igFdr := strings.Split(*igFldPtr, ",")

	var fiExt []string
	if *filtExPtr != "" {
		fiExt = strings.Split(*filtExPtr, ",")
	}

	// Force add .git
	igFdr = append(igFdr, ".git")

	posiArgs := flag.Args()

	// positional arg as path
	var path string
	if flag.NArg() == 0 {
		p, e := os.Getwd()
		if e != nil {
			os.Exit(1)
		}
		path = p
	} else {
		path = posiArgs[0]
	}

	nodes, err := buildTree(path, igExt, igFdr, fiExt)
	if err != nil {
		fmt.Printf("Error: %v", err)
		os.Exit(1)
	}

	// printTree(nodes, "")
	formatedTree := formatTree(nodes)
	formatedCodeBase := formatFiles(nodes)
	res := formatedTree + "\n" + formatedCodeBase

	printRes := res
	if *tokenLimit >= 0 {
		counted, err := truncateTokens(res, *tokenLimit)
		if err != nil {
			fmt.Printf("Error: %v", err)
			os.Exit(1)
		}
		printRes = *counted
		// fmt.Printf("%d", counted)
	}
	fmt.Printf("%s\n", printRes)
}
