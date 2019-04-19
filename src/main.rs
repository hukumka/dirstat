//! Simple utility to display total size of all files in directory
//!
//! USAGE:
//!    dirstat [FLAGS] <input_dir>
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -n, --nice       Display result in readable units
//!     -V, --version    Prints version information
//! 
//! ARGS:
//!     <input_dir>    Path to file or directory you want check stats on
//!

use structopt::StructOpt;
use std::path::PathBuf;
use std::io;
use std::fmt;
use std::fs;

#[derive(StructOpt)]
struct Opt{
    /// Path to file or directory you want check stats on
    #[structopt(parse(from_os_str))]
    input_dir: PathBuf,   
    /// Display result in readable units
    #[structopt(short="-n", long="--nice")]
    human_readable: bool
}

fn main() {
    let opt = Opt::from_args();
    let mut stats = Stats::new();
    stats.collect(&opt.input_dir).unwrap();
    if opt.human_readable{
        println!("Size of {:?} is {}", opt.input_dir, SizeWithUnit::new(stats.size));
    }else{
        println!("Size of {:?} is {}", opt.input_dir, stats.size);
    }
}

struct SizeWithUnit{
    size: f64,
    unit: SizeUnit,
}

impl fmt::Display for SizeWithUnit{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        use SizeUnit::*;
        let prefix = match self.unit{
            B => "B",
            KB => "KB",
            MB => "MB",
            GB => "GB",
            TB => "TB",
        };
        write!(f, "{:.2}{}", self.size, prefix)
    }
}

impl SizeWithUnit{
    fn new(bytes: u64) -> Self{
        if bytes < (1<<10){
            Self{size: bytes as f64, unit: SizeUnit::B}
        }else if bytes < (1<<20){
            let size = (bytes as f64) / (1u64<<10) as f64;
            Self{size, unit: SizeUnit::KB}
        }else if bytes < (1<<30){
            let size = (bytes as f64) / (1u64<<20) as f64;
            Self{size, unit: SizeUnit::MB}
        }else if bytes < (1<<40){
            let size = (bytes as f64) / (1u64<<30) as f64;
            Self{size, unit: SizeUnit::GB}
        }else{
            let size = (bytes as f64) / (1u64<<40) as f64;
            Self{size, unit: SizeUnit::TB}
        }
    }
}

enum SizeUnit{
    B,
    KB,
    MB,
    GB,
    TB,
}

struct Stats{
    /// Total size of directory in bytes
    size: u64,
}

impl Default for Stats{
    fn default() -> Self{
        Self::new()
    }
}

impl Stats{
    fn new() -> Self{
        Self{size: 0}
    }

    fn collect(&mut self, path: &PathBuf) -> io::Result<()>{
        let metadata = fs::metadata(path)?;
        if metadata.is_file(){
            self.collect_meta(&metadata);
        }
        if metadata.is_dir() && !metadata.file_type().is_symlink(){
            self.collect_dir(path)?;
        }
        Ok(())
    }

    fn collect_dir(&mut self, path: &PathBuf) -> io::Result<()>{
        for inner in fs::read_dir(path)?{
            let inner = inner?;
            let metadata = inner.metadata()?;
            if metadata.is_file(){
                self.collect_meta(&metadata);
            }
            if metadata.is_dir() && !metadata.file_type().is_symlink(){
                self.collect_dir(&inner.path())?;
            }
        }
        Ok(())
    }

    fn collect_meta(&mut self, metadata: &fs::Metadata){
        self.size += metadata.len();
    }
}

