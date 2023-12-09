### My first API with Rust and Actix-Web framework 🤓

This is a simple API that I made to learn Rust and Actix-Web framework. It's a simple CRUD API that basically return infos about the server and a fake user list.

### How to run

First of all, you need to have Rust installed in your machine. You can download it [here](https://www.rust-lang.org/tools/install). Also, install Cargo, the Rust package manager. You can download it [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

- After that, you need to clone this repository and run the following command in the root folder:

```bash
cargo run
```

- Now you can access the API at http://localhost:9091

### Endpoints

- **GET** `/info` - Returns the server info
- **GET** `/products` - Returns a list of fake products (10 items)
- **GET** `/ping` - Returns a simple pong message

### Future improvements

- [ ] Add a database
- [ ] Add a user authentication
- [ ] Add a user registration

These are some improvements that I want to make in the future. If you have any suggestions, feel free to open an issue or a pull request.
