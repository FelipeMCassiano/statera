<div align='center'>
  <h1>⚖️Statera⚖️</h1>
  <p>Statea is a high-performance load balancer written in Rust 🦀.</p>
  <img src='https://img.shields.io/github/languages/top/FelipeMCassiano/statera' alt='GitHub top language' />
  <img src='https://img.shields.io/github/last-commit/FelipeMCassiano/statera' alt='GitHub last commit' />  
</div>

## 🌟 Features
- **Round-Robin Algorithm**: Fair and equal distribution of traffic to prevent server overload.
- **TOML Configuration**: Easy-to-use and human-readable configuration files 📄.
- **High Performance**: Built with Rust for safety and speed 🚀.

## ⚙️ Configuration
⚠️ The statera configuration file name is strictly determined to `statera.toml`.

Below is the structure of the configuration file you’ll need to set up:

```toml
# Defines the Statera port
port = "9999"

# Server configurations
[[servers]]
name = "api 1"
host = "localhost"
port = "8080"

[[servers]]
name = "api 2"
host = "localhost"
port = "8081"

# Health check settings (Optional)
[health_check]
interval = 10 # Interval for health check (in seconds)
endpoint = "/health"
max_failures = 2

# Ssl/tls statera settings (Optional)
[ssl]
certificate = "certificate-file.pem" # needs to be .pem
key = "key-file.pem" # needs to be .pem

```

## 🐳 How to Use with Docker Compose
To run Statera using Docker Compose, you can use the following configuration in your `docker-compose.yml` file:

```yaml
...
services:
  statera:
    image: felipecassiano/statera:latest
    volumes:
      - your-path-to-statera.toml/statera.toml:/usr/local/bin/statera.toml
      - your-path-to-ssl-key.pem:/usr/local/bin/key-file.pem
      - your-path-to-ssl-certificate.pem://usr/local/bin/certificate-file.pem
    ports:
      - "9999:9999" # This is an example port mapping
...
```

## 📜 License
Distributed under the MIT License. See [LICENSE](LICENSE) for more information.
