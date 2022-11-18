"use strict";
const _ = {};
_["+"] = function(a, b) {
  return a + b;
};
_["-"] = function(a, b) {
  return a - b;
};
_["*"] = function(a, b) {
  return a * b;
};
_["/"] = function(a, b) {
  return a / b;
};
export default { ["+"]: _["+"], ["-"]: _["-"], ["*"]: _["*"], ["/"]: _["/"] };
//# sourceMappingURL=math.js.map
