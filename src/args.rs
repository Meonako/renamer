use clap::Parser;

#[derive(Parser)]
pub struct Arguments {
    #[arg(id = "REPLACE")]
    /// Text that will be replace
    pub text_to_replace_pos: Option<String>,
    
    #[arg(id = "TEXT")]
    /// New text that will replace the old one
    pub replace_with_pos: Option<String>,
    
    #[arg(id = "DIRECTORY")]
    /// Directory to scan
    pub dir_to_read_pos: Option<String>,

    #[arg(short, long = "dir")]
    /// Directory to scan
    pub dir_to_read: Option<String>,
    
    #[arg(short = 'r', long = "replace")]
    /// Text that will be replace
    pub text_to_replace: Option<String>,
    
    #[arg(short = 'w', long = "with")]
    /// New text that will replace the old one
    pub replace_with: Option<String>,
}