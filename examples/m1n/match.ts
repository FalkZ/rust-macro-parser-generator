import util, { pipe } from "@std/util";
import math from "@std/math";

const self = {};

function matchSimple(v) {
  return util["match"](v, (_) => {
    if (_ == 1) {
      return 1;
    } else if (_ == `a string`) {
      return _;
    } else if (_ == -2) {
      return 2;
    }
  });
}

function matchExample(v) {
  return pipe(v)
    .op(util["match"], (_) => {
      if (_ == 1) {
        return 1;
      } else if (_ == `a string`) {
        return _;
      } else if (_ == -2) {
        return 2;
      }
    })
    .op(math["+"], 2)
    .end();
}
export default { ...self };
