#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::mem::Handle;
use neon::js::JsInteger;

fn fib(call: Call) -> JsResult<JsInteger> {
  let scope = call.scope;
  let index: Handle<JsInteger> = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>());
  let index: i32 = index.value() as i32;
  let result: i32 = fibonacci(index);
  Ok(JsInteger::new(scope, result))
}

fn fibonacci(n: i32) -> i32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2)
  }
}

register_module!(m, {
  m.export("fib", fib)
});
