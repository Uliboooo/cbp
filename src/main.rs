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

const DEFAULT_IGNORE_FDR: [&str; 3] = [".git", ".target", "node_modules"];

impl Cli {
    fn resolve(self) -> Config {
        let path = self
            .path
            .map(PathBuf::from)
            .unwrap_or(std::env::current_dir().unwrap().to_path_buf());
        let ignore_e: Vec<PathBuf> = [self.ignore_extensions.unwrap_or_default().as_slice()]
            .concat()
            .iter()
            .map(PathBuf::from)
            .collect();

        let ignore_folder = [
            self.ignore_folders.unwrap_or_default().as_slice(),
            &DEFAULT_IGNORE_FDR.map(|f| f.to_string()),
        ]
        .concat()
        .iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>();

        let tree_opt = TreeOption::from(self.tree.unwrap_or("show".to_string()));

        Config {
            path,
            ignore_extensions: ignore_e,
            ignore_folders: ignore_folder,
            tree: tree_opt,
        }
    }
}

struct Config {
    path: PathBuf,
    ignore_extensions: Vec<PathBuf>,
    ignore_folders: Vec<PathBuf>,
    tree: TreeOption,
}

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

impl FileNode {
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

fn make_tree<T: AsRef<Path>>(root_path: T) -> FileNode {
    let p = root_path.as_ref().to_path_buf();
    if p.is_file() {
        FileNode::File(p)
    } else {
        let dirs = fs::read_dir(&p)
            .unwrap()
            .filter_map(|f| f.ok())
            .map(|f| f.path())
            .map(make_tree)
            .collect::<Vec<_>>();
        FileNode::Dir(p, dirs)
    }
}

fn run(cli: Cli) {
    let config = cli.resolve();
    let tree = make_tree(config.path);
    // TODO: impl .map() for FileNode
}

fn main() {
    let cli = Cli::parse();
    run(cli);
}
