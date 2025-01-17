# Clothe Match

This is a platform where users upload their clothes and the system suggests outfits based on the user's preferences.
We provide a web interface and native mobile and desktops apps.

---

## Features

- Multiplatform: Web, Android, iOS, Windows, Linux, and macOS.
- Fast: The API is built with Rust and the frontend with Astro/Svelte.
- Secure: Passwords are hashed with Argon2.

---

## Requirements

You will need the following items installed run this project:

- [Docker](https://v2.tauri.app/reference/cli/) (optional, to run the application in containers)
- [Git](https://upx.github.io/)
- [Postman](https://crates.io/crates/cargo-watch) (optional, to conduct API tests)  
- [Python](https://github.com/astral-sh/uv) - to run the build system and the machine learning model
- [Rust](https://www.postgresql.org/) - to build the backend
- [bun](https://bun.sh/) - Fast JS runtime/bundler
- [PostgreSQL](https://www.rust-lang.org/tools/install) - Database
- [uv](https://www.python.org/downloads/) - Modern Python package manager
- [cargo](https://v2.tauri.app/reference/cli/)-watch - Reload backend dev server on changes
- [upx](https://v2.tauri.app/reference/cli/) - Compress the app binary
- [tauri](https://v2.tauri.app/reference/cli/)-cli - Build the multiplatform native app

---

## How to Run

### Clone the Repository

```bash
git clone https://github.com/lucascompython/PDSDI-TP
cd PDSDI-TP
```

---

### Run the Application with Docker

```bash
# Run production mode
docker compose up

# Run development mode
docker compose -f docker-compose-dev.yml up
```

### Run the Application without Docker

```bash
./make.py --help
```

---

## Test the API Routes

The API can be tested using tools like Postman. All routes and request details are documented in the [API.md](API.md) file. Additionally, you can import the [Postman collection file](resources/PDSDI.postman_collection.json) included in the repository to facilitate testing.

---

## Docker Image

The API Docker image is available at: [Docker Hub](https://hub.docker.com/r/l33tlsl/clothe_match_backend):

```bash
docker pull l33tlsl/clothe_match_backend
```

---

## Project Structure

- API.md: Detailed documentation of the API routes.
- docker-compose.yml: Configuration to run the application in Docker containers.
- Resources/*: Additional resources for the project:
  - Reports
  - Presentations
  - Postman Collection
  - Test Data  
  - Database Model
  - Mockups
- app/: Source code of the application.
- frontend/: Source code of the frontend.
- backend/: Source code of the backend.
- models/: Source code of the machine learning model.
- cbf/: Source code of the CBF (Custom Binary Format) algorithm.
- make.py: Main script of the custom build system for the project.

---

## Related Repositories

In the development of this project, we conducted some studies and experiments that resulted in other repositories and open source contributions:

- [argon2-bench-rust](https://github.com/lucascompython/argon2-bench-rust) - Study on the performance of different password hashing algorithms in Rust.
- [parking_lot_vs_std](https://github.com/lucascompython/parking_lot_vs_std) - Study on the performance of different synchronization primitives in Rust.
- [argon2-kdf](https://github.com/lucascompython/argon2-kdf) - Fastest Argon2 implementation in Rust, that we contributed to.

## License

This project is under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for more details.

---

### Academic Information

- **Curricular Unit**: Serviços Distribuídos
- **Professor**: Wenderson Wanzeller
- **Authors**: Lucas de Linahres; Guilherme Sousa
- **Ano Letivo**: 2024/2025
- **University**: Instituto Politécnico de Viana do Castelo
