extern crate lolbench_support ; # [ test ] fn end_to_end ( ) { lolbench_support :: end_to_end_test ( "crossbeam_epoch_0_4_0" , "defer :: single_alloc_defer_free" , "defer-single-alloc-defer-free.rs" , "defer-single-alloc-defer-free" , ) ; }