<div>
  <h4 align="center">
    LaTeX Compiler Server
  </h4>
</div>

## Run in Docker

1. Pull and Run the Image

```bash
docker pull ghcr.io/sendcv/latex_compiler:latest
docker run -p 3000:3000 latex_compiler
```

2. The service should be available in your local network by addressing `http://localhost:3000`
