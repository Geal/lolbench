extern crate crossbeam_epoch_0_4_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; crossbeam_epoch_0_4_0 :: defer :: single_defer ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crossbeam_epoch_0_4_0 :: defer :: single_defer ( & mut crit ) ; }