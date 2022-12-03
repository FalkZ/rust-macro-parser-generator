import util, { pipe } from "@std/util";
import math from "@std/math";

const self = {};

function operatorExample(a, b) {
  return pipe(a)
    .op(math["!"])
    .op(math["+"], b)
    .op(math["-"], a)
    .op(math["+"], b)
    .end();
}

function operatorExampleBrackets(a, b) {
  return and(not(math["+"](math["!"](a), b)), b);
}

function operatorExampleMixed(a, b) {
  return pipe(math["!"](a))
    .op(and, b)
    .op(or, a)
    .op(math["+"], b)
    .end();
}
export default { ...self };
