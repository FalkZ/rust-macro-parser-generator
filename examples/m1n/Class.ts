import util, { pipe } from "@std/util";
import math from "@std/math";

export class Class {
  private readonly PI = pipe(a)
    .op(math["+"], b)
    .op(math["+"], 0.012).end;

  public Pi = 123;

  private fn(a, b) {
    return pipe(a).op(math["+"], b).end;
  }

  private one(b) {
    return pipe(a)
      .op(math["+"], b)
      .op(math["+"], c).end;
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
      fn(
        pipe(a).op(
          math["+"],
          pipe(b).op(math["-"], c).end
        ).end,
        123.2
      )
    );
  }
}
