import util, { pipe } from "@std/util";
import math from "@std/math";

let val = 0;

const self = {};

function setVal(v) {
  return pipe(v).op(util["assign"], (_) => {
    val = _;
  }).end;
}
export default { ...self };
