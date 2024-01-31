# Socket Programming: Simple Application Layer Protocol

## A simple application layer protocol design for the project assignment

### How To Run

1. Install Rust into your computer using these methods.
    - Windows: Download via [https://www.rust-lang.org/tools/install] and choose rustup-init.exe to match with your CPU architecture.
    - Mac: You can use homebrew to download using

    ``` bash
    brew install rust
    ```

    - UNIX/curl: You can install rust on UNIX Operating System using

    ``` bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. Open your terminal and run the program.
    1. Run the server first using `cargo run --bin server`
    2. Run the client using `cargo run --bin client`

3. You will these following output formats.
    - On server.rs you will see

    ```text
    Starting server for recieve 1 request...
    I got this message from client: <ProtocolName> <RequestRoute> <ProtocolVersion>

    Received a message, closing ...
    ```

    - On client.rs you will see

    ```text
    Starting client...
    I got this message from server: PTC_VER: <ProtocolVersion>
    STATUS_CODE: <StatusCode>
    STATUS_PHRASE: <StatusPhrase>
    ```

4. server.rs and client.rs is closing itself.
Congratulations! You have successfully run the code.
