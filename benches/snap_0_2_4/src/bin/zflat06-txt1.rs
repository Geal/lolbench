extern crate snap_0_2_4 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; snap_0_2_4 :: zflat06_txt1 ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; snap_0_2_4 :: zflat06_txt1 ( & mut crit ) ; }