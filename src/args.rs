use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct GenomeArgs {
    /// please provide the path to the alignment file
    pub alignment_arg: String,
    /// please provide the region start
    pub genome_start: usize,
    /// please provide the region end
    pub genome_end: usize
}
