use natord::compare;
use std::path::{Path, PathBuf};

pub struct Navigation {
    pub file_list: Vec<PathBuf>,
    pub current_index: usize,
}

impl Navigation {
    pub fn from_path(current_path: &Path) -> Self {
        let dir = current_path.parent().unwrap_or(Path::new("."));
        let supported_extensions = ["jpg", "jpeg", "png", "gif", "webp", "bmp", "ico", "tiff"];
        let mut file_list = Vec::new();

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if supported_extensions.contains(&ext.to_lowercase().as_str()) {
                            file_list.push(path);
                        }
                    }
                }
            }
        }

        file_list.sort_by(|a, b| {
            compare(
                &a.file_name().unwrap().to_string_lossy(),
                &b.file_name().unwrap().to_string_lossy(),
            )
        });

        let current_index = file_list
            .iter()
            .position(|p| p == current_path)
            .unwrap_or(0);

        Self {
            file_list,
            current_index,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.file_list.is_empty()
    }

    pub fn current_path(&self) -> Option<&PathBuf> {
        self.file_list.get(self.current_index)
    }

    pub fn navigate_previous(&mut self) {
        if self.file_list.is_empty() {
            return;
        }
        self.current_index = if self.current_index == 0 {
            self.file_list.len() - 1
        } else {
            self.current_index - 1
        };
    }

    pub fn navigate_next(&mut self) {
        if self.file_list.is_empty() {
            return;
        }
        self.current_index = if self.current_index + 1 >= self.file_list.len() {
            0
        } else {
            self.current_index + 1
        };
    }
}
