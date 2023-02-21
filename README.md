# PSL_Assessment

1. It imports the required standard Rust libraries.

2. It defines the cache TTL as a constant.

3. It defines a `CacheEntry` struct that represents a cache entry and a `Cache` struct that manages the cache.

4. The `Cache` struct has two methods `get` and `put`. `get` method returns the cache entry if it exists and the entry is not expired, and `put` method stores the cache entry in the cache with the current timestamp.

5. It defines the `handle_client` function that receives a TCP stream, reads the request, forwards it to the origin server, reads the response, and writes it back to the client. Before writing the response back to the client, it stores the response in the cache for the requested path.

6. It defines the `main` function that listens to incoming requests, and for each request, checks if there's a cached response for the requested path. If there is, it writes the cached response to the client. Otherwise, it handles the request using the `handle_client` function.