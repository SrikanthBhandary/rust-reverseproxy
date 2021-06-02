# rust-reverseproxy
Simple reverse proxy for http. This can be used in the scneario where your microservice is behind gateway and not exposed to public.

User can configure the system by simple json config.  Currently, the api gateway is authorizing the request by checking the jwt token expiry. Only the valid tokens are validated and forwaded to the other services. User can configure the JWT_SECRET_KEY in the environment.

Written the code for example purpose. It can be used in production, only to serve http requests. Route fetching can be improved, and any contribution to this is appreciated.

```
{
  "/data_service": "http://127.0.0.1:9002",
  "/repor_tservice": "http://127.0.0.1:9003"
}
```

### How to Run ?
PORT=13900 JWT_SECRET_KEY="Srikanth1234*" CONFIG_PATH="config.json" ./reverse-proxy
