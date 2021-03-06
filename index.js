const rust = require('./native');

function fib(n) {
  if (n === 1 || n === 2) {
    return 1;
  }
  return fib(n - 1) + fib(n - 2);
}

// js
console.time('node');
console.log(fib(40));
console.timeEnd('node');

// rust
console.time('rust');
console.log(rust.fib(40));
console.timeEnd('rust');