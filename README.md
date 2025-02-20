# Go Installation and Upgrade Tool 🚀

A tool for installing or upgrading an existing go installation written in rust 🔥

<img src="assets/demo.gif" />

The only thing this does is follow the steps described at 
[Go Installation](https://go.dev/doc/install).
It is definitely not a replacement for a 
package manager or manual installation. It is intended for developers who 
enjoy using a Rust tool to install Go.

## Installation 📥

Download the precompiled binary from the releases.

### Usage

```sh
go-installer
```

## Developing 💻

> Note: This tool is originally developed for Linux amd64.

Fetch dependencies:

```sh
cargo update
```

To run for development:

```sh
cargo run
```

To run with specific flags, e.g., to use version 1.23.4 of Go:

```sh
cargo run -- --version 1.23.4
```

### Manual QA 🧪

Requirements:

- Docker 🐳
- Docker Compose 🐙
- At least 300MB for the Docker image and the download

We will use an isolated environment to test the tool without affecting the 
development computer. By default, DEBUG verbose level logging is enabled.

1. Execute `test.sh`.
2. Once inside the Docker container, execute the following commands and take 
  note:
  - Check the current shell profile:
   ```sh
   cat /etc/profile
   ```
  - Check the binaries currently in the system:
   ```sh
   ls /usr/local
   ```
3. Run the installation process:
  ```sh
  go-installer --version 1.23.4
  ```
4. Verify that the PATH environment variable includes the Go bin path:
  ```sh
  cat /etc/profile
  ```
5. Confirm that the only change in the installation path `/usr/local` is the 
  addition of the `go` directory:
  ```sh
  ls /usr/local
  ```
6. Re-source the current shell:
  ```sh
  source /etc/profile
  ```
7. Check the Go version:
  ```sh
  go version
  ```
  The output should be 1.23.4.
8. Run the upgrade process to a new version:
  ```sh
  go-installer --version 1.24.0
  ```
9. Verify that the PATH environment variable still includes the bin path and 
  that the line is not duplicated:
  ```sh
  cat /etc/profile
  ```
10. Confirm that the only change in the installation path `/usr/local` is the 
   presence of the `go` directory:
  ```sh
  ls /usr/local
  ```
11. Check the Go version again, which should now be 1.24.0:
  ```sh
  go version
  ```

## TODO 📝

- [x] Implement a working version of the tool
- [x] Add a demo to the README.md
- [ ] Clean up the code 🧹
- [ ] Automate QA 🤖
- [ ] Make shell PATH export configurable 🔧
- [ ] Automate the release process 🚀
- [ ] Build multi-arch binaries 🏗️
- [ ] Get the latest version automagically 🪄
