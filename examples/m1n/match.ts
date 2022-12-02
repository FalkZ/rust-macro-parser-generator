import util, { pipe, match } from "@std/util";
import math from "@std/math";

const self = {};

function matchExample(v) {
  return pipe(v)
    .op(match, (_) => {
      if (_ == 1) {
        return 1;
      } else if (_ == `a string`) {
        return 0;
      } else if (_ == 2) {
        return 2;
      }
    })
    .op(math["+"], 2).end;
}
export default { ...self };
