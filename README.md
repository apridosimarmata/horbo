![Horbo Logo](./assets/images/horbo-github.png)

# 🐃 Horbo

**Horbo** is a lightweight service discovery system written in Rust.  
It helps services register themselves and enables other services to find them efficiently — using consistent hashing under the hood.

> "Horbo" means buffalo in Bataknese — a strong, grounded creature that represents resilience and structure. 🐃

---

## ✨ Features

- 📝 **Service Registration** — services can register with IP and metadata
- 🔍 **Service Lookup** — fast lookup using consistent hashing
- ♻️ **Singleton Mapping Layer** — internal service mapper is a thread-safe singleton
- ⚙️ Built in **Rust** for speed, safety, and reliability


---

## 🧠 Design Highlights

- 💡 Uses [consistent hashing](https://en.wikipedia.org/wiki/Consistent_hashing) to evenly distribute keys across services
- 🧵 Thread-safe singleton for mapping state
- 🚀 Ready to expand with health checks, gossip sync, or peer awareness

---

## 📦 Installation

```bash
git clone https://github.com/your-username/horbo.git
cd horbo
cargo build --release
```

---

## 🧪 Usage (Example)

```rust
use horbo::discovery::{register, lookup};

register("service-A", "192.168.1.10");
let endpoint = lookup("some-key");
```

> More usage examples coming soon...

---

## 🛣 Roadmap

- [x] Service Registration
- [x] Service Lookup
- [x] Health Check (Heartbeat)
- [ ] Gossip Protocol for syncing
- [ ] REST API or gRPC interface
- [ ] CLI or Web Dashboard

Please see [diagrams](./FEATURES.md) for details.
---

## 📷 Logo

<p align="center">
  <img src="./assets/horbo-logo.png" alt="Horbo Logo" width="200"/>
</p>

---

## 🤝 Contributing

Contributions are welcome!  
Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

---

## 📄 License

MIT License — see [LICENSE](./LICENSE) file.

---

## 🙏 Acknowledgements

- Inspired by systems like [Consul](https://www.consul.io/) and [Eureka](https://github.com/Netflix/eureka)
- Built with 💛 in Rust
