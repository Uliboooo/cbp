use std::{
    env::args,
    fs,
    path::{self, Path, PathBuf},
};

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

fn main() {
    let path = &args().collect::<Vec<_>>()[1];

    let tree = make_tree(path);
    let print_tree = tree.print();
    let vt = tree.flat();
    let codes = read_sources(vt);

    println!("File Tree");
    println!("\n{}\n\n", print_tree);
    println!("{}", codes);
}
