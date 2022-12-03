import util, { pipe } from "@std/util";
import math from "@std/math";

export class Class {
  private readonly PI = pipe(a)
    .op(math["+"], b)
    .op(math["+"], 0.012)
    .end();

  public Pi = 123;

  private fn(a, b) {
    return math["+"](a, b);
  }

  private one(b) {
    return pipe(a)
      .op(math["+"], b)
      .op(math["+"], c)
      .end();
  }

  public zero() {
    return (() => {
      throw new Error("hello 2");
    })();
  }

  private multiline() {
    return console.log(
      `test space`,
      `multi

    line`
    );
  }

  private nesting() {
    return one(
      fn(math["+"](a, math["-"](b, c)), 123.2)
    );
  }
}
