# DigiByte Ordinals Indexer

ℹ️ This is a fork adapted for DigiByte blockchain from [reallyshadydev/wonky-ord-digibyte](https://github.com/reallyshadydev/wonky-ord-digibyte)

## Prerequisites
You will need to launch your own DigiByte node and have it fully synced. Follow these steps to set up:

1. Download and install the latest version from [DigiByte Core](https://github.com/digibyte/digibyte/releases)
   1. We have tested the indexer with DigiByte Core.

2. Start your DigiByte node with these recommended flags:
   ```shell
   digibyted -txindex -rpcuser=your_username -rpcpassword=your_password -rpcport=12024 -rpcallowip=0.0.0.0/0 -rpcbind=127.0.0.1
   ```
   - Make sure your DigiByte node is fully synced before starting the indexer
   - ‼️ **IMPORTANT**: Replace `your_username` and `your_password` with secure credentials

## Building the Indexer

1. Clone the repository:
```shell
git clone https://github.com/reallyshadydev/wonky-ord-digibyte.git
cd wonky-ord-digibyte
```

2. Build the release version:
```shell
cargo build --release
```

## Running the Indexer

### Basic Setup
```shell
export RUST_LOG=info

# Create data directory
mkdir -p /mnt/ord-node/orddigi

# Start Indexing + Server
./target/release/ord \
    --first-inscription-height=0 \
    --rpc-url=http://your_username:your_password@127.0.0.1:12024 \
    --data-dir=/mnt/ord-node/orddigi \
    --index-transactions \
    --nr-parallel-requests=16 \
    server --http-port 8169
```

Important Parameters:
- `--index-transactions`: Stores transaction data for better API performance
- `--nr-parallel-requests=16`: Configures parallel requests to your RPC Server
- `--data-dir`: Specifies where the indexer stores its data
- `--http-port`: The port where the server will listen (default: 8169)

### Docker Support

If you prefer using Docker, you can build and run the indexer in a container:

1. Build the Docker image:
```shell
docker build -t digibyte/ord-indexer .
```

2. Start the container:
```shell
docker compose up -d
```

3. Stop the container (with proper timeout for database closure):
```shell
docker compose stop -t 600
docker compose down
```

## Storage Requirements

The database size will depend on the indexing options enabled and the current blockchain size. Make sure you have adequate storage space available.

## API Documentation

You can find the API documentation in the [openapi.yaml](https://github.com/reallyshadydev/wonky-ord-digibyte/blob/master/openapi.yaml) file. 
The most convenient way to view the API documentation is to use the [Swagger Editor](https://editor.swagger.io/).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under [CC0-1.0 license](https://github.com/reallyshadydev/wonky-ord-digibyte/blob/master/LICENSE)

## Repository

For more information, source code, and updates, visit the [GitHub repository](https://github.com/reallyshadydev/wonky-ord-digibyte).
