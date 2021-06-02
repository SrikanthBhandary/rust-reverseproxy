# rust-reverseproxy
Simple reverse proxy for http. This can be used in the scneario where your microservice is behind gateway and not exposed to public.

User can configure the system by simple json config. 

```
{
  "/data_service": "http://127.0.0.1:9002",
  "/repor_tservice": "http://127.0.0.1:9003"
}
```

### How to Run ?
PORT=13900 JWT_SECRET_KEY="Srikanth1234*" CONFIG_PATH="config.json" ./reverse-proxy
