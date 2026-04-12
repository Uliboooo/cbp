use clap::Parser;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    path: Option<String>,

    #[arg(long, value_delimiter = ',')]
    ignore_extensions: Option<Vec<String>>,

    #[arg(long, value_delimiter = ',')]
    ignore_folders: Option<Vec<String>>,

    #[arg(short, long)]
    tree: Option<String>,
}

const DEFAULT_IGNORE_FDR: [&str; 3] = [".git", "target", "node_modules"];

impl Cli {
    fn resolve(self) -> Config {
        let path = self
            .path
            .map(PathBuf::from)
            .unwrap_or(std::env::current_dir().unwrap().to_path_buf());

        let ignore_e = [self.ignore_extensions.unwrap_or_default().as_slice()]
            .concat()
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>();

        let default = DEFAULT_IGNORE_FDR.map(PathBuf::from).map(|f| path.join(f));
        let ignore_folder = [
            self.ignore_folders.unwrap_or_default().as_slice(),
            &default.map(|f| f.to_string_lossy().to_string()),
        ]
        .concat()
        .iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>();

        // let ignore_folder = Vec::new();

        let tree_opt = TreeOption::from(self.tree.unwrap_or("show".to_string()));

        Config {
            path,
            ignore_extensions: ignore_e,
            ignore_folders: ignore_folder,
            tree: tree_opt,
        }
    }
}

#[derive(Debug, Clone)]
struct Config {
    path: PathBuf,
    ignore_extensions: Vec<String>,
    ignore_folders: Vec<PathBuf>,
    tree: TreeOption,
}

#[derive(Debug, Clone)]
enum TreeOption {
    None,
    Only,
    Show,
}

impl From<String> for TreeOption {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "only" => TreeOption::Only,
            "show" => TreeOption::Show,
            _ => TreeOption::None,
        }
    }
}

const SEPARATOR: &str = "------------------------------------------------";

fn read_sources<T: AsRef<Path>>(path_list: Vec<T>) -> String {
    path_list
        .iter()
        .filter(|f| f.as_ref().to_path_buf().exists())
        .map(|f| f.as_ref().to_path_buf())
        .map(|f| (f.to_string_lossy().to_string(), fs::read_to_string(f)))
        .filter_map(|f| match f.1 {
            Ok(file_cont) => Some((f.0, file_cont)),
            Err(_) => None,
        })
        .map(|f| format!("{}\n{}\n{}\n{}\n", f.0, SEPARATOR, f.1, SEPARATOR))
        .collect::<String>()
}

#[derive(Debug, Clone)]
enum FileNode {
    File(PathBuf),
    Dir(PathBuf, Vec<FileNode>),
}

struct FileNoteIter<'a> {
    stack: Vec<&'a FileNode>,
}

impl<'a> Iterator for FileNoteIter<'a> {
    type Item = &'a PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        match node {
            FileNode::File(path_buf) => Some(path_buf),
            FileNode::Dir(path_buf, file_nodes) => {
                for c in file_nodes.iter().rev() {
                    self.stack.push(c);
                }
                Some(path_buf)
            }
        }
    }
}

impl FileNode {
    fn iter(&self) -> FileNoteIter {
        FileNoteIter { stack: vec![self] }
    }

    fn print_helper(&self, deps: u32) -> String {
        match self {
            FileNode::File(path_buf) => format!(
                "{}{}",
                { "  ".repeat(deps as usize) },
                path_buf.to_string_lossy()
            ),
            FileNode::Dir(path_buf, files) => format!(
                "{}{}\n{}",
                { "  ".repeat(deps as usize) },
                path_buf.to_string_lossy(),
                {
                    files
                        .iter()
                        .map(|f| f.print_helper(deps + 1))
                        .collect::<Vec<String>>()
                        .join("\n")
                }
            ),
        }
    }

    fn print(&self) -> String {
        self.print_helper(0)
    }

    fn flat(self) -> Vec<PathBuf> {
        match self {
            FileNode::File(path_buf) => vec![path_buf],
            FileNode::Dir(_path_buf, files) => files
                .iter()
                .cloned()
                .flat_map(|f| f.flat())
                .collect::<Vec<_>>(),
        }
    }
}

fn make_tree<T: AsRef<Path>>(root_path: T, config: &Config) -> FileNode {
    let p = root_path.as_ref().to_path_buf();
    if p.is_file() {
        FileNode::File(p)
    } else {
        let dirs = fs::read_dir(&p)
            .unwrap()
            .filter_map(|f| f.ok())
            .map(|f| f.path())
            .filter(|p| match p.extension() {
                Some(e) => !config
                    .ignore_extensions
                    .contains(&e.to_string_lossy().to_string()),
                None => true,
            })
            .filter(|f| {
                if f.is_dir() {
                    !config.ignore_folders.contains(f)
                } else {
                    true
                }
            })
            .map(|f| make_tree(f, &config.clone()))
            .collect::<Vec<_>>();
        FileNode::Dir(p, dirs)
    }
}

fn run(cli: Cli) {
    let config = cli.resolve();
    let tree = make_tree(config.path.clone(), &config);

    // TODO: impl .map() for FileNode
    println!("result\n{:?}", tree);
}

fn main() {
    let cli = Cli::parse();
    run(cli);
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{Cli, make_tree};

    fn resolve_path() -> PathBuf {
        std::env::current_dir().map(|f| f.join("test")).unwrap()
    }
    #[test]
    fn filter_test() {
        let cli = Cli {
            path: Some(resolve_path().to_string_lossy().to_string()),
            ignore_extensions: Some(vec!["txt".to_string()]),
            ignore_folders: None,
            tree: None,
        };

        let conf = cli.resolve();
        println!("\n{:?}\n\n", conf);
        let tree = make_tree(conf.path.clone(), &conf);
        // println!("{:?}", tree);
        println!("{}", tree.print());
    }
}
