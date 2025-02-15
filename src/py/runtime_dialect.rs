use crate::allocator::{Allocator, NodePtr};
use crate::cost::Cost;
use crate::dialect::Dialect;
use crate::err_utils::err;
use crate::more_ops::op_unknown;
use crate::py::f_table::{f_lookup_for_hashmap, FLookup};
use crate::reduction::Response;
use std::collections::HashMap;

pub struct RuntimeDialect {
    f_lookup: FLookup,
    quote_kw: Vec<u8>,
    apply_kw: Vec<u8>,
    strict: bool,
}

impl RuntimeDialect {
    pub fn new(
        op_map: HashMap<String, Vec<u8>>,
        quote_kw: Vec<u8>,
        apply_kw: Vec<u8>,
        strict: bool,
    ) -> RuntimeDialect {
        RuntimeDialect {
            f_lookup: f_lookup_for_hashmap(op_map),
            quote_kw,
            apply_kw,
            strict,
        }
    }
}

impl Dialect for RuntimeDialect {
    fn op(
        &self,
        allocator: &mut Allocator,
        o: NodePtr,
        argument_list: NodePtr,
        max_cost: Cost,
    ) -> Response {
        let b = &allocator.atom(o);
        if b.len() == 1 {
            if let Some(f) = self.f_lookup[b[0] as usize] {
                return f(allocator, argument_list, max_cost);
            }
        }
        if self.strict {
            err(o, "unimplemented operator")
        } else {
            op_unknown(allocator, o, argument_list, max_cost)
        }
    }

    fn quote_kw(&self) -> &[u8] {
        &self.quote_kw
    }

    fn apply_kw(&self) -> &[u8] {
        &self.apply_kw
    }
}
