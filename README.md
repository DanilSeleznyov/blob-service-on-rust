# Blob Service usage

## 1. If raising on docker compose

**Start the service:**
```bash
cd blob-service-on-rust
make up
```

**Create a blob:**
```bash
make create FILE=photo1.jpg TAGS=tag1,tag2
make create FILE=photo2.jpg TAGS=tag2,tag3

# with full set of data
make create FILE=photo1.jpg TAGS=tag1 METADATA=source=web OWNER_ID=alice ACCESS_LEVEL=private
```

**Update a blob:**
```bash
# update only tags
make update ID=<uuid> TAGS=tag2,another_tag

# update data and rename
make update ID=<uuid> FILE=photo3.jpg NAME=photo3-renamed.jpg
```

**Get a blob:**
```bash
# save as a file
make get ID=<uuid> OUT=downloaded.jpg

# get blob info in terminal
make get ID=<uuid>
```

**Get a list of blobs:**
```bash
make list

# applying filters
make list NAME=photo TAGS=tag1 LIMIT=5
```

**Delete a blob:**
```bash
make delete ID=<uuid>
```

**Stop a service:**
```bash
make down
```

**Remove all data:**
```bash
make clean
```

## 2. If raising from a usual binary

**Start the service:**
```bash
cd blob-service-on-rust
docker compose up -d postgres
cargo run --bin server
```

**Create a blob:**
```bash
cargo run --bin client -- create --file photo1.jpg --tags tag1,tag2
cargo run --bin client -- create --file photo2.jpg --tags tag2,tag3

# with metadata, owner and access level
cargo run --bin client -- create --file photo1.jpg --tags tag1 --metadata source=web --owner-id alice --access-level private
```

**Update a blob:**
```bash
# update only tags
cargo run --bin client -- update --id <uuid> --tags tag2,another_tag

# update data and rename
cargo run --bin client -- update --id <uuid> --file photo3.jpg --name photo3-renamed.jpg
```

**Get a blob:**
```bash
# save as a file
cargo run --bin client -- get --id <uuid> --out downloaded.jpg

# get blob info in terminal
cargo run --bin client -- get --id <uuid>
```

**Get a list of blobs:**
```bash
cargo run --bin client -- list

# applying filters
cargo run --bin client -- list --name photo --tags tag1 --limit 5
```

**Delete a blob:**
```bash
cargo run --bin client -- delete --id <uuid>
```

**Remove all data:**
```bash
make clean
```