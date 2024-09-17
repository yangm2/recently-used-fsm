use clap::Parser;
use core::str;
use itertools::Itertools;
use std::{collections::HashSet, fmt, fs, num::NonZeroU8, vec::Vec};

#[derive(Debug, clap::Parser)] // requires `derive` feature
#[command(bin_name = "rufsm")]
#[command(about = "example CLI", long_about = None)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to generated file
    #[arg(short = 'o', long, value_hint = clap::ValueHint::FilePath)]
    output_path: std::path::PathBuf,

    /// Module name
    #[arg(short = 'm', long, default_value_t = String::from("Arbiter"))]
    module_name: String,

    /// Number of elements to order
    #[arg(short = 'e', default_value_t = NonZeroU8::new(3).unwrap(), long)]
    elements: NonZeroU8,
    
    /// Asynchronously reset FSM ... otherwise synchronous reset is default
    #[arg(long, default_value_t = false)]
    asynch_reset: bool,

    /// MRU Arbitration policy ... otherwise LRU is default
    #[arg(long, default_value_t = false)]
    mru: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Element {
    name: String,
}

impl Element {
    fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone)]
struct Permutation {
    name: String,
    inner: Vec<Element>,
}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.inner.len() < 2 {
            return Err::<_, std::fmt::Error>(std::fmt::Error {});
        }

        // write!(f, "{}::", self.name);
        let repr = self.inner.iter().map(|x| x.name.clone()).join("_");
        write!(f, "{}::{repr}", self.name)
    }
}

fn main() {
    let args = Args::parse();

    let cnt_u8: u8 = args.elements.into();
    let mut hset: HashSet<Element> = HashSet::with_capacity(cnt_u8.into());

    for i in 1..=cnt_u8 {
        let e = Element::new(format!("e{i}"));
        hset.insert(e);
    }

    let p = hset.into_iter().permutations(cnt_u8 as usize).sorted();

    println!("number of permutations = {}", p.len());

    let mut file_data: Vec<String> = vec![];

    for (idx, perm) in p.enumerate() {
        let state = Permutation {
            name: format!("s{}", idx),
            inner: perm,
        };

        println!("{}", state);

        file_data.push(format!("// {}\n", state));
    }

    // write out file
    let _ = fs::write(&args.output_path, file_data.concat());
}
