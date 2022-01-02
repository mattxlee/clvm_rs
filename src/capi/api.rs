use std::ptr::copy;
use std::vec::Vec;
use std::cmp::max;

use crate::allocator::Allocator;
use crate::chia_dialect::ChiaDialect;
use crate::node::Node;
use crate::run_program::{run_program, STRICT_MODE};
use crate::serialize::{node_from_bytes, node_to_bytes};

#[no_mangle]
pub extern "C" fn hello() -> i32 {
    100
}

#[no_mangle]
pub unsafe extern "C" fn run_chia_program(
    _prog: *const u8,
    _prog_len: usize,
    _args: *const u8,
    _args_len: usize,
    _res: *mut u8,
    _res_len: &mut usize,
    _max_cost: u64,
    _flags: u32,
) -> u64 {
    let mut prog: Vec<u8> = Vec::new();
    copy(_prog, prog.as_mut_ptr(), _prog_len);
    let mut args: Vec<u8> = Vec::new();
    copy(_args, args.as_mut_ptr(), _args_len);
    let mut allocator = Allocator::new();
    let prog2 = node_from_bytes(&mut allocator, &prog).unwrap();
    let args2 = node_from_bytes(&mut allocator, &args).unwrap();
    let dialect = ChiaDialect::new((_flags & STRICT_MODE) != 0);
    let redu = run_program(&mut allocator, &dialect, prog2, args2, _max_cost, None).unwrap();
    let result = redu.1;
    let res_node = Node::new(&allocator, result);
    let res_bytes = node_to_bytes(&res_node).unwrap();
    let res_len = res_bytes.len();
    let len = max(res_len, *_res_len);
    copy(res_bytes.as_ptr(), _res, len);
    *_res_len = len;
    redu.0
}
