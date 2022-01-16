#include "core_opts.h"

#include "costs.h"

namespace chia {

OpResult op_if(CLVMObjectPtr args) {
  if (ListLen(args) != 3) {
    throw std::runtime_error("i takes exactly 3 arguments");
  }
  auto [first, r] = Pair(args);
  if (IsNull(first)) {
    return std::make_tuple(IF_COST, First(Rest(r)));
  }
  return std::make_tuple(IF_COST, First(r));
}

OpResult op_cons(CLVMObjectPtr args) {
  if (ListLen(args) != 2) {
    throw std::runtime_error("c takes exactly 2 arguments");
  }
  return std::make_tuple(CONS_COST, ToSExp(First(args), First(Rest(args))));
}

OpResult op_first(CLVMObjectPtr args) {
  if (ListLen(args) != 1) {
    throw std::runtime_error("f takes exactly 1 argument");
  }
  return std::make_tuple(FIRST_COST, First(First(args)));
}

OpResult op_rest(CLVMObjectPtr args) {
  if (ListLen(args) != 1) {
    throw std::runtime_error("r takes exactly 1 argument");
  }
  return std::make_tuple(REST_COST, Rest(First(args)));
}

OpResult op_listp(CLVMObjectPtr args) {
  if (ListLen(args) != 1) {
    throw std::runtime_error("l takes exactly 1 argument");
  }
  return std::make_tuple(LISTP_COST, ListP(First(args)) ? ToTrue() : ToFalse());
}

OpResult op_raise(CLVMObjectPtr args) {
  throw std::runtime_error("clvm raise");
}

OpResult op_eq(CLVMObjectPtr args) {
  if (ListLen(args) != 2) {
    throw std::runtime_error("= takes exactly 2 arguments");
  }
  auto a0 = First(args);
  auto a1 = First(Rest(args));
  if (a0->GetNodeType() == NodeType::Pair ||
      a0->GetNodeType() == NodeType::Pair) {
    throw std::runtime_error("= on list");
  }
  auto b0 = Atom(a0);
  auto b1 = Atom(a1);
  Cost cost{EQ_BASE_COST};
  cost += (b0.size() + b1.size()) * EQ_COST_PER_BYTE;
  return std::make_tuple(cost, b0 == b1 ? ToTrue() : ToFalse());
}

}  // namespace chia
