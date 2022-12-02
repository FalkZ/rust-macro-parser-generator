import util, { pipe, match } from "@std/util";
import math from "@std/math";

const PI = pipe(a)
  .op(math["+"], b)
  .op(math["+"], 0.012).end;

export let Pi = 123;

const self = {};

function fn(a, b) {
  return pipe(a).op(math["+"], b).end;
}

function one(b) {
  return console.log(b);
}

export function zero() {
  return pipe(a).op(math["+"], b).end;
}
export default { ...self, zero };
