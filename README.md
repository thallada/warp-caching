# warp-caching

This is an example web API that expands on the [todos example in the warp 
repo](https://github.com/seanmonstar/warp/blob/09f7a71306a46131d8ece7c49c32d21b52e90700/examples/todos.rs) 
in order to test different methods of caching. I want to know how easy the 
different methods are to implement and how they rank in performance.

Besides caching, I have added these features on-top of the `todos.rs` example:

* Split up the monolithic `todos.rs` into separate module files.
* Better rejections using [`anyhow`](https://crates.io/crates/anyhow) errors 
  that are JSON-ified into HTTP responses using 
  [`http-api-problem`](https://crates.io/crates/http-api-problem).
* Added the `Environment` struct that's just a cloneable container for handles 
  to the database and caches.

## Caching methods

Here I will detail the different methods in this repo and how they performed.

### No cache

Endpoint: `/no_cache/todos`

The baseline. This is approximately the same as the todos example in the warp 
repo. Since it uses an in-memory database instead of a real database, it 
actually performs the best out of all of the other options. But, if we were to 
use a real database or add some delay to each faked database access then we 
would see that this is actually the slowest option.

### Database result LRU cache

TODO

Similar to [identity_cache](https://github.com/Shopify/identity_cache) in the 
Ruby on Rails world.

### HTTP response LRU cache

Endpoint: `/lru_cache/todos`

This option also uses the [lru](https://crates.io/crates/lru) crate wrapped in a 
tokio Mutex, but it stores the entire HTTP response as bytes instead of just the 
response from the database queries.

warp's `Response` is not cloneable, which prevents it from being stored in a 
cache directly, so I made a `CachedResponse` struct that serializes from a warp 
`Reply` and cache it instead. When fetching from the cache, the `CachedResponse` 
is converted back into a warp `Response` with `Response::builder()`.

### Redis cache

TODO

Should probably use the [redis](https://crates.io/crates/redis) crate.

### Other??

I'd like to test out as many of these options that I can:

* [nginx proxy response cache](https://docs.nginx.com/nginx/admin-guide/content-cache/content-caching/)
* [memcached](https://memcached.org/)
* [cached crate](https://lib.rs/crates/cached). I'd like to see if it could 
  reduce a lot of the boilerplate in my own LRU cache implementation. I tried it 
  briefly before, but ran into some issues.

## Benchmarks

TODO
