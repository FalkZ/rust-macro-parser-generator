import util, { pipe } from '@std/util';
import math from '@std/math';





 const self = {['+'](a, b) { return (a + b);},

['-'](a, b) { return (a - b);},

['*'](a, b) { return (a * b);},

['/'](a, b) { return (a / b);}}




export default { ...self, }