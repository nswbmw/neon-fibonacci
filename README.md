## neon-fibonacci

A fibonacci demo of [Neon](https://github.com/neon-bindings/neon).

### Environment

```
rustc -V => rustc 1.20.0-nightly (229d0d326 2017-06-23)
node -v => v8.1.1
```

```js
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
```

print

```
102334155
node: 1006.893ms
102334155
rust: 221.644ms
```

