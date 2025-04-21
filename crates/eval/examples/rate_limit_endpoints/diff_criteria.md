1. The main.go changes introduce rate-limited endpoints by creating them via `MakeEndpoints` and passing them to both HTTP and gRPC servers instead of directly using the service. This includes:
   - Adding endpoint creation before server startup
   - Modifying HTTP server to use endpoints
   - Modifying gRPC server to use endpoints
2. The server_grpc.go changes update the gRPC server implementation to use the provided endpoints instead of creating them internally. This affects both hash and validate endpoints which are now taken from the Endpoints struct rather than being created via makeHashEndpoint/makeValidateEndpoint.
3. The server_http.go changes mirror the gRPC server changes, modifying the HTTP server to use endpoints from the Endpoints struct rather than creating them internally for both hash and validate routes.
4. The service.go changes include:
   - Renaming makeHashEndpoint to MakeHashEndpoint and making it public
   - Renaming makeValidateEndpoint to MakeValidateEndpoint and making it public
   - Adding new MakeEndpoints function that creates rate-limited endpoints using a token bucket (5 requests per second)
   - Adding new dependencies for rate limiting (kitrl and ratelimit packages)
   - The Endpoints struct remains the same but is now populated with rate-limited versions of the endpoints
