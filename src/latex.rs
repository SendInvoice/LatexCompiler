use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, LatexError>;

#[derive(Error, Debug)]
pub enum LatexError {
    #[error("I/O error: {0}")]
    IoError(io::Error),
    #[error("LaTeX compilation error: {0}")]
    CompilationError(String),
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
}

impl From<io::Error> for LatexError {
    fn from(error: io::Error) -> Self {
        LatexError::IoError(error)
    }
}

pub struct LatexCompiler {
    work_dir: PathBuf,
    latex_engine: String,
}

impl LatexCompiler {
    pub fn new() -> Result<Self> {
        let work_dir = env::temp_dir().join("latex_compile");
        fs::create_dir_all(&work_dir)?;

        let latex_engine = if Self::check_command("pdflatex") {
            "pdflatex".to_string()
        } else if Self::check_command("xelatex") {
            "xelatex".to_string()
        } else if Self::check_command("lualatex") {
            "lualatex".to_string()
        } else {
            return Err(LatexError::ToolNotFound(
                "No LaTeX engine found. Please install TeX Live or MiKTeX".to_string(),
            ));
        };

        Ok(LatexCompiler {
            work_dir,
            latex_engine,
        })
    }

    fn check_command(cmd: &str) -> bool {
        Command::new(cmd)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub fn compile_text(&self, latex_content: &str, output_name: &str) -> Result<PathBuf> {
        let tex_file = self.work_dir.join(format!("{output_name}.tex"));
        let mut file = fs::File::create(&tex_file)?;
        file.write_all(latex_content.as_bytes())?;
        file.flush()?;

        self.compile_file(&tex_file)
    }

    pub fn compile_file<P: AsRef<Path>>(&self, tex_file: P) -> Result<PathBuf> {
        let tex_path = tex_file.as_ref();
        let file_stem = tex_path
            .file_stem()
            .ok_or_else(|| LatexError::CompilationError("Invalid file name".to_string()))?;
        let output = Command::new(&self.latex_engine)
            .arg("-interaction=nonstopmode")
            .arg("-output-directory")
            .arg(&self.work_dir)
            .arg(tex_path)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(LatexError::CompilationError(format!(
                "LaTeX compilation failed:\nSTDOUT:\n{stdout}\nSTDERR:\n{stderr}"
            )));
        }

        let pdf_file = self
            .work_dir
            .join(format!("{}.pdf", file_stem.to_string_lossy()));
        if !pdf_file.exists() {
            return Err(LatexError::CompilationError(
                "PDF file was not generated".to_string(),
            ));
        }

        Ok(pdf_file)
    }
}
