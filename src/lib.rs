use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FileSystem {
    index_nodes: Vec<IndexNode>,
}

#[derive(Debug, PartialEq)]
pub enum IndexNode {
    File {
        virtual_path: Box<Path>,
        disk_path: Box<Path>,
        binary: bool,
    }
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            index_nodes: vec![]
        }
    }

    pub fn get_base_path(&self) -> PathBuf {
        PathBuf::new()
    }

    /// Create a file in the filesystem.
    /// If the file already exists, this function will return `None`.
    ///
    /// * `disk_path` - Path to file on the hosts system.
    /// * `virtual_path` - Internal path representation.
    /// * `as_bytes` - Should the file be loaded into a byte array?
    pub fn add_file(&mut self, disk_path: PathBuf, virtual_path: &PathBuf, as_bytes: bool) -> Option<usize> {
        let mut virtual_path = virtual_path.clone();
        virtual_path.push("foo");
        virtual_path.set_file_name(
            disk_path.file_name().unwrap().to_os_string().into_string().unwrap());

        let index_node = IndexNode::File{
            virtual_path: Box::from(virtual_path),
            disk_path: Box::from(disk_path),
            binary: as_bytes,
        };

        if self.index_nodes.contains(&index_node) {
            return None;
        }

        self.index_nodes.push(index_node);

        Some(self.index_nodes.len())
    }

    /// Generates standalone code representing the filesystem.
    pub fn code(&self, canonicalize: bool) -> String {
        let mut code = "".to_owned();

        code += "pub mod files {";
        for inode in &self.index_nodes {
            match inode {
                IndexNode::File { virtual_path, disk_path, binary } => {
                    let virtual_path_id = format!("{}", virtual_path.display())
                        .as_str()
                        .replace(".", "_")
                        .replace("/", "_")
                        .to_uppercase();

                    let include_path = match canonicalize {
                        true => std::fs::canonicalize(disk_path).unwrap(),
                        false => disk_path.to_path_buf(),
                    };

                    code += format!(
                        r#"pub const FILE_{}: {} = include_{}!("../{}");"#,
                        virtual_path_id,
                        if *binary { "&[u8]" } else { "&str" },
                        if *binary { "bytes" } else { "str" },
                        include_path.display(),
                    ).as_str();
                }
            }
        }

        code += "}";

        #[cfg(feature = "format")]
        {
            use rust_format::Formatter;
            code = rust_format::RustFmt::default().format_str(code).unwrap();
        }

        return code;
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::FileSystem;

    #[test]
    fn functionality() {
        let mut fs = FileSystem::new();
        let mut image_directory = fs.get_base_path();

        image_directory.push("assets");
        image_directory.push("imgs");

        let puppy = fs.add_file(PathBuf::from("puppy.png"), &image_directory);

        panic!("{}", fs.code(false));
    }
}

