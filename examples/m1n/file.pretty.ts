import util, { pipe } from "@std/util";
import math from "@std/math";

const PI = pipe(a)
  .op(math["+"], b)
  .op(math["+"], 0.012)
  .end();

export let Pi = 123;

const self = {};

function fn(a, b) {
  return math["+"](a, b);
}

function one(b) {
  return console.log(b);
}

export function zero() {
  return math["+"](a, b);
}
export default { ...self, zero };
