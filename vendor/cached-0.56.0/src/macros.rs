/*!
Declarative macros for defining functions that wrap a static-ref cache object.

### `cached!` and `cached_key!` Usage & Options:

There are several options depending on how explicit you want to be. See below for a full syntax breakdown.


1.) Using the shorthand will use an unbounded cache.


```rust,no_run
#[macro_use] extern crate cached;

/// Defines a function named `fib` that uses a cache named `FIB`
cached!{
    FIB;
    fn fib(n: u64) -> u64 = {
        if n == 0 || n == 1 { return n }
        fib(n-1) + fib(n-2)
    }
}
# pub fn main() { }
```


2.) Using the full syntax requires specifying the full cache type and providing
    an instance of the cache to use. Note that the cache's key-type is a tuple
    of the function argument types. If you would like fine grained control over
    the key, you can use the `cached_key!` macro.
    The following example uses a `SizedCache` (LRU):

```rust,no_run
#[macro_use] extern crate cached;

use std::thread::sleep;
use std::time::Duration;
use cached::SizedCache;

/// Defines a function `compute` that uses an LRU cache named `COMPUTE` which has a
/// size limit of 50 items. The `cached!` macro will implicitly combine
/// the function arguments into a tuple to be used as the cache key.
cached!{
    COMPUTE: SizedCache<(u64, u64), u64> = SizedCache::with_size(50);
    fn compute(a: u64, b: u64) -> u64 = {
        sleep(Duration::new(2, 0));
        return a * b;
    }
}
# pub fn main() { }
```


3.) The `cached_key` macro functions identically, but allows you to define the
    cache key as an expression.

```rust,no_run
#[macro_use] extern crate cached;

use std::thread::sleep;
use std::time::Duration;
use cached::SizedCache;

/// Defines a function named `length` that uses an LRU cache named `LENGTH`.
/// The `Key = ` expression is used to explicitly define the value that
/// should be used as the cache key. Here the borrowed arguments are converted
/// to an owned string that can be stored in the global function cache.
cached_key!{
    LENGTH: SizedCache<String, usize> = SizedCache::with_size(50);
    Key = { format!("{}{}", a, b) };
    fn length(a: &str, b: &str) -> usize = {
        let size = a.len() + b.len();
        sleep(Duration::new(size as u64, 0));
        size
    }
}
# pub fn main() { }
```

4.) The `cached_result` and `cached_key_result` macros function similarly to `cached`
    and `cached_key` respectively but the cached function needs to return `Result`
    (or some type alias like `io::Result`). If the function returns `Ok(val)` then `val`
    is cached, but errors are not. Note that only the success type needs to implement
    `Clone`, _not_ the error type. When using `cached_result` and `cached_key_result`,
    the cache type cannot be derived and must always be explicitly specified.

```rust,no_run
#[macro_use] extern crate cached;

use cached::UnboundCache;

/// Cache the successes of a function.
/// To use `cached_key_result` add a key function as in `cached_key`.
cached_result!{
   MULT: UnboundCache<(u64, u64), u64> = UnboundCache::new(); // Type must always be specified
   fn mult(a: u64, b: u64) -> Result<u64, ()> = {
        if a == 0 || b == 0 {
            return Err(());
        } else {
            return Ok(a * b);
        }
   }
}
# pub fn main() { }
```

----

```rust,ignore
#[macro_use] extern crate cached;
use std::thread::sleep;
use std::time::Duration;
use cached::RedisCache;

cached! {
    UNBOUND_REDIS: RedisCache<u32, u32> = RedisCache::new();
    fn cached_redis(n: u32) -> u32 = {
        sleep(Duration::new(3, 0));
        n
    }
}

cached! {
    TIMED_REDIS: RedisCache<u32, u32> = RedisCache::with_lifespan(Duration::from_secs(2));
    fn cached_timed_redis(n: u32) -> u32 = {
        sleep(Duration::new(3, 0));
        n
    }
}
# pub fn main() { }
```


----

## Syntax

The common macro syntax is:


```rust,ignore
cached_key!{
    CACHE_NAME: CacheType = CacheInstance;
    Key = KeyExpression;
    fn func_name(arg1: arg_type, arg2: arg_type) -> return_ty = {
        // do stuff like normal
        return_type
    }
}
```

Where:

- `CACHE_NAME` is the unique name used to hold a `static ref` to the cache
- `CacheType` is the full type of the cache
- `CacheInstance` is any expression that yields an instance of `CacheType` to be used
  as the cache-store, followed by `;`
- When using the `cached_key!` macro, the "Key" line must be specified. This line must start with
  the literal tokens `Key = `, followed by an expression that evaluates to the key, followed by `;`
- `fn func_name(arg1: arg_type) -> return_type` is the same form as a regular function signature, with the exception
  that functions with no return value must be explicitly stated (e.g. `fn func_name(arg: arg_type) -> ()`)
- The expression following `=` is the function body assigned to `func_name`. Note, the function
  body can make recursive calls to its cached-self (`func_name`).


# Fine grained control using `cached_control!`

The `cached_control!` macro allows you to provide expressions that get plugged into key areas
of the memoized function. While the `cached` and `cached_result` variants are adequate for most
scenarios, it can be useful to have the ability to customize the macro's functionality.

```rust,no_run
#[macro_use] extern crate cached;

use cached::UnboundCache;

/// The following usage plugs in expressions to make the macro behave like
/// the `cached_result!` macro.
cached_control!{
    CACHE: UnboundCache<String, String> = UnboundCache::new();

    // Use an owned copy of the argument `input` as the cache key
    Key = { input.to_owned() };

    // If a cached value exists, it will bind to `cached_val` and
    // a `Result` will be returned containing a copy of the cached
    // evaluated body. This will return before the function body
    // is executed.
    PostGet(cached_val) = { return Ok(cached_val.clone()) };

    // The result of executing the function body will be bound to
    // `body_result`. In this case, the function body returns a `Result`.
    // We match on the `Result`, returning an early `Err` if the function errored.
    // Otherwise, we pass on the function's result to be cached.
    PostExec(body_result) = {
        match body_result {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };

    // When inserting the value into the cache we bind
    // the to-be-set-value to `set_value` and give back a copy
    // of it to be inserted into the cache
    Set(set_value) = { set_value.clone() };

    // Before returning, print the value that will be returned
    Return(return_value) = {
        println!("{}", return_value);
        Ok(return_value)
    };

    fn can_fail(input: &str) -> Result<String, String> = {
        let len = input.len();
        if len < 3 { Ok(format!("{}-{}", input, len)) }
        else { Err("too big".to_string()) }
    }
}
# pub fn main() {}
```


 */

#[macro_export]
macro_rules! cached {
    // Use default cached::Cache
    ($cachename:ident;
     fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        cached!(
            $cachename : $crate::UnboundCache<($($argtype),*), $ret> = $crate::UnboundCache::new();
            fn $name($($arg : $argtype),*) -> $ret = $body
        );
    };

    // Use a specified cache-type and an explicitly created cache-instance
    ($cachename:ident : $cachetype:ty = $cacheinstance:expr ;
     fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        static $cachename: $crate::once_cell::sync::Lazy<::std::sync::Mutex<$cachetype>>
            = $crate::once_cell::sync::Lazy::new(|| ::std::sync::Mutex::new($cacheinstance));

        #[allow(unused_parens)]
        pub fn $name($($arg: $argtype),*) -> $ret {
            let key = ($($arg.clone()),*);
            {
                let mut cache = $cachename.lock().unwrap();
                let res = $crate::Cached::cache_get(&mut *cache, &key);
                if let Some(res) = res { return res.clone(); }
            }
            let val = (||$body)();
            let mut cache = $cachename.lock().unwrap();
            $crate::Cached::cache_set(&mut *cache, key, val.clone());
            val
        }
    };

    ($cachename:ident : $cachetype:ty = $cacheinstance:expr ;
     async fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:block) => {
        static $cachename: $crate::once_cell::sync::Lazy<::std::sync::Mutex<$cachetype>>
            = $crate::once_cell::sync::Lazy::new(|| ::std::sync::Mutex::new($cacheinstance));

        #[allow(unused_parens)]
        pub async fn $name($($arg: $argtype),*) -> $ret {
            let key = ($($arg.clone()),*);
            {
                let mut cache = $cachename.lock().unwrap();
                let res = $crate::Cached::cache_get(&mut *cache, &key);
                if let Some(res) = res { return res.clone(); }
            }
            // run the function and cache the result
            async fn inner($($arg: $argtype),*) -> $ret $body
            let val = inner($($arg),*).await;

            let mut cache = $cachename.lock().unwrap();
            $crate::Cached::cache_set(&mut *cache, key, val.clone());
            val
        }
    };
}

#[macro_export]
macro_rules! cached_key {
    // Use a specified cache-type and an explicitly created cache-instance
    ($cachename:ident : $cachetype:ty = $cacheinstance:expr ;
     Key = $key:expr;
     fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        static $cachename: $crate::once_cell::sync::Lazy<::std::sync::Mutex<$cachetype>>
            = $crate::once_cell::sync::Lazy::new(|| ::std::sync::Mutex::new($cacheinstance));

        #[allow(unused_parens)]
        pub fn $name($($arg: $argtype),*) -> $ret {
            let key = $key;
            {
                let mut cache = $cachename.lock().unwrap();
                let res = $crate::Cached::cache_get(&mut *cache, &key);
                if let Some(res) = res { return res.clone(); }
            }
            let val = (||$body)();
            let mut cache = $cachename.lock().unwrap();
            $crate::Cached::cache_set(&mut *cache, key, val.clone());
            val
        }
    };

    ($cachename:ident : $cachetype:ty = $cacheinstance:expr ;
     Key = $key:expr;
     async fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        static $cachename: $crate::once_cell::sync::Lazy<::std::sync::Mutex<$cachetype>>
            = $crate::once_cell::sync::Lazy::new(|| ::std::sync::Mutex::new($cacheinstance));

        #[allow(unused_parens)]
        pub async fn $name($($arg: $argtype),*) -> $ret {
            let key = $key;
            {
                let mut cache = $cachename.lock().unwrap();
                let res = $crate::Cached::cache_get(&mut *cache, &key);
                if let Some(res) = res { return res.clone(); }
            }
            // run the function and cache the result
            async fn inner($($arg: $argtype),*) -> $ret $body
            let val = inner($($arg),*).await;
            let mut cache = $cachename.lock().unwrap();
            $crate::Cached::cache_set(&mut *cache, key, val.clone());
            val
        }
    };
}

#[macro_export]
macro_rules! cached_result {
    // Unfortunately it's impossible to infer the cache type because it's not the function return type
    ($cachename:ident : $cachetype:ty = $cacheinstance:expr ;
     fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        static $cachename: $crate::once_cell::sync::Lazy<::std::sync::Mutex<$cachetype>>
            = $crate::once_cell::sync::Lazy::new(|| ::std::sync::Mutex::new($cacheinstance));

        #[allow(unused_parens)]
        pub fn $name($($arg: $argtype),*) -> $ret {
            let key = ($($arg.clone()),*);
            {
                let mut cache = $cachename.lock().unwrap();
                let res = $crate::Cached::cache_get(&mut *cache, &key);
                if let Some(res) = res { return Ok(res.clone()); }
            }

            // Store return in temporary typed variable in case type cannot be inferred
            let ret : $ret = (||$body)();
            let val = ret?;

            let mut cache = $cachename.lock().unwrap();
            $crate::Cached::cache_set(&mut *cache, key, val.clone());
            Ok(val)
        }
    };

    ($cachename:ident : $cachetype:ty = $cacheinstance:expr ;
     async fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        static $cachename: $crate::once_cell::sync::Lazy<::std::sync::Mutex<$cachetype>>
            = $crate::once_cell::sync::Lazy::new(|| ::std::sync::Mutex::new($cacheinstance));

        #[allow(unused_parens)]
        pub async fn $name($($arg: $argtype),*) -> $ret {
            let key = ($($arg.clone()),*);
            {
                let mut cache = $cachename.lock().unwrap();
                let res = $crate::Cached::cache_get(&mut *cache, &key);
                if let Some(res) = res { return Ok(res.clone()); }
            }

            // run the function and cache the result
            async fn inner($($arg: $argtype),*) -> $ret $body
            let val = inner($($arg),*).await?;

            let mut cache = $cachename.lock().unwrap();
            $crate::Cached::cache_set(&mut *cache, key, val.clone());
            Ok(val)
        }
    };
}

#[macro_export]
macro_rules! cached_key_result {
    // Use a specified cache-type and an explicitly created cache-instance
    ($cachename:ident : $cachetype:ty = $cacheinstance:expr ;
     Key = $key:expr;
     fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        static $cachename: $crate::once_cell::sync::Lazy<::std::sync::Mutex<$cachetype>>
            = $crate::once_cell::sync::Lazy::new(|| ::std::sync::Mutex::new($cacheinstance));

        #[allow(unused_parens)]
        pub fn $name($($arg: $argtype),*) -> $ret {
            let key = $key;
            {
                let mut cache = $cachename.lock().unwrap();
                let res = $crate::Cached::cache_get(&mut *cache, &key);
                if let Some(res) = res { return Ok(res.clone()); }
            }

            // Store return in temporary typed variable in case type cannot be inferred
            let ret : $ret = (||$body)();
            let val = ret?;

            let mut cache = $cachename.lock().unwrap();
            $crate::Cached::cache_set(&mut *cache, key, val.clone());
            Ok(val)
        }
    };

    ($cachename:ident : $cachetype:ty = $cacheinstance:expr ;
     Key = $key:expr;
     async fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        static $cachename: $crate::once_cell::sync::Lazy<::std::sync::Mutex<$cachetype>>
            = $crate::once_cell::sync::Lazy::new(|| ::std::sync::Mutex::new($cacheinstance));

        #[allow(unused_parens)]
        pub async fn $name($($arg: $argtype),*) -> $ret {
            let key = $key;
            {
                let mut cache = $cachename.lock().unwrap();
                let res = $crate::Cached::cache_get(&mut *cache, &key);
                if let Some(res) = res { return Ok(res.clone()); }
            }

            // run the function and cache the result
            async fn inner($($arg: $argtype),*) -> $ret $body
            let val = inner($($arg),*).await?;

            let mut cache = $cachename.lock().unwrap();
            $crate::Cached::cache_set(&mut *cache, key, val.clone());
            Ok(val)
        }
    };
}

#[macro_export]
macro_rules! cached_control {
    // Use a specified cache-type and an explicitly created cache-instance
    ($cachename:ident : $cachetype:ty = $cacheinstance:expr ;
     Key = $key:expr;
     PostGet($cached_value:ident) = $post_get:expr;
     PostExec($body_value:ident) = $post_exec:expr;
     Set($set_value:ident) = $pre_set:expr;
     Return($ret_value:ident) = $return:expr;
     fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        static $cachename: $crate::once_cell::sync::Lazy<::std::sync::Mutex<$cachetype>>
            = $crate::once_cell::sync::Lazy::new(|| ::std::sync::Mutex::new($cacheinstance));

        #[allow(unused_parens)]
        pub fn $name($($arg: $argtype),*) -> $ret {
            let key = $key;
            {
                let mut cache = $cachename.lock().unwrap();
                let res = $crate::Cached::cache_get(&mut *cache, &key);
                if let Some($cached_value) = res {
                    $post_get
                }
            }
            let $body_value = (||$body)();
            let $set_value = $post_exec;
            let mut cache = $cachename.lock().unwrap();
            $crate::Cached::cache_set(&mut *cache, key, $pre_set);
            let $ret_value = $set_value;
            $return
        }
    };

    ($cachename:ident : $cachetype:ty = $cacheinstance:expr ;
     Key = $key:expr;
     PostGet($cached_value:ident) = $post_get:expr;
     PostExec($body_value:ident) = $post_exec:expr;
     Set($set_value:ident) = $pre_set:expr;
     Return($ret_value:ident) = $return:expr;
     async fn $name:ident ($($arg:ident : $argtype:ty),*) -> $ret:ty = $body:expr) => {
        static $cachename: $crate::once_cell::sync::Lazy<::std::sync::Mutex<$cachetype>>
            = $crate::once_cell::sync::Lazy::new(|| ::std::sync::Mutex::new($cacheinstance));

        #[allow(unused_parens)]
        pub async fn $name($($arg: $argtype),*) -> $ret {
            let key = $key;
            {
                let mut cache = $cachename.lock().unwrap();
                let res = $crate::Cached::cache_get(&mut *cache, &key);
                if let Some($cached_value) = res {
                    $post_get
                }
            }
             // run the function and cache the result
            async fn inner($($arg: $argtype),*) -> $ret $body
            let $body_value = inner($($arg),*).await?;
            let $set_value = $post_exec;
            let mut cache = $cachename.lock().unwrap();
            $crate::Cached::cache_set(&mut *cache, key, $pre_set);
            let $ret_value = $set_value;
            $return
        }
    };
}
