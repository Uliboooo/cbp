package main

import (
	"flag"
	"fmt"
	"golang.org/x/term"
	"log"
	"os"
	"path/filepath"
	"slices"
	"strings"
)

type FileNode struct {
	Name     string
	Path     string
	IsDir    bool
	Children []*FileNode
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

func buildTree(path string, ignoreExt []string, ignoreFdr []string) (*FileNode, error) {
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
		childNode, err := buildTree(chilePath, ignoreExt, ignoreFdr)
		if err != nil {
			log.Printf("errr in process child node: %v", err)
			continue
		}
		if childNode != nil {
			node.Children = append(node.Children, childNode)
		}
	}

	return node, nil
}

func printTree(node *FileNode, prefix string) {
	if node == nil {
		return
	}

	// 表示する名前を作成
	name := node.Name
	if node.IsDir {
		// ディレクトリ名は装飾
		name = fmt.Sprintf("\033[34m%s/\033[0m", name) // 青色で表示
	}

	fmt.Printf("%s%s\n", prefix, name)

	for i, child := range node.Children {
		isLast := i == len(node.Children)-1

		newPrefix := prefix + ""
		if isLast {
			newPrefix = prefix + ""
		}

		currentPrefix := "├─ "
		if isLast {
			currentPrefix = "└─ "
		}

		printTree(child, newPrefix+currentPrefix)
	}
}

func mulStr(str string, mul int) string {
	var s string
	for range mul {
		s = s + str

	}
	return s
}

func printFiles(node *FileNode) {
	var x int
	fd := int(os.Stdout.Fd())
	if !term.IsTerminal(fd) {
		x = 20
	}
	width, _, _ := term.GetSize(fd)
	x = width / 2

	if node == nil {
		return
	}

	if !node.IsDir {
		cont, _ := os.ReadFile(node.Path)
		line := mulStr("─", x)
		fmt.Printf("%s\n%s\n%s\n%s\n", line, node.Path, line, &cont)
	}

	// fmt.Printf("%s\n")
	for _, child := range node.Children {
		if child.IsDir {
			printFiles(child)
		} else {
			cont, err := os.ReadFile(child.Path)
			if err != nil {
				continue
			}
			line := mulStr("─", x)
			fmt.Printf("%s\n%s\n%s\n%s\n", line, child.Path, line, &cont)
		}
	}

}

func main() {
	igExtPtr := flag.String("ignore_ext", "", "ignore extensions")
	igFldPtr := flag.String("ignore_fld", "", "ignore folders")
	includeGitPrt := flag.Bool("git", false, "")

	// pathPtr := flag.String("path", "", "")

	flag.Parse()

	includeGit := *includeGitPrt

	igExt := strings.Split(*igExtPtr, ",")
	igFdr := strings.Split(*igFldPtr, ",")

	if !includeGit {
		igFdr = append(igFdr, ".git")
	}

	posiArgs := flag.Args()

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

	nodes, err := buildTree(path, igExt, igFdr)
	if err != nil {
		fmt.Printf("Error: %v", err)
		os.Exit(1)
	}

	printTree(nodes, "")
	printFiles(nodes)
}
