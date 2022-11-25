# Key Logger

## Prerequisites

### Shared

```cmd
cargo install sea-orm-cli
```

### Client

```cmd
cargo install tauri-cli
```

## Migration

```cmd
sea-orm-cli.exe migrate -d .\packages\migration\ up
```

## Entities

```cmd
sea-orm-cli.exe generate entity -l -o .\packages\entity\src\
```
