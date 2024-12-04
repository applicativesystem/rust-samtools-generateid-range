# rust-samtools-generateid-range
 
 - rust-samtools generateid range
 - rust samtools generateid range will only generate the fastidx for the specified range for the use with the other rust-samtools. 
 - The difference between this and the rust-samtools-generateid is that rust-samtools-generateid for all the identifiers whereas in this you can specify a start and stop coordinate and it will generate only the fastidx for those range and not for the whole sam file. 
 - general note: Incase of Golang and RUST, please see the last commit message and if it says compiled binary then it is completed or else still in development version.
 
 ```
 cargo build 
 
 ```

 ```
  ./target/debug/rust-samtools-generateid-range 
         ./sample-files/alignreads-metagenomics.sam  540849 613960
 ```
 
 Gaurav Sablok
