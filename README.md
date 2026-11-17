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
```

**Update a blob:**
```bash
make update ID=<uuid> FILE=photo3.jpg TAGS=tag2,another_tag
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
make up
```

**Create a blob:**
```bash
make create FILE=photo1.jpg TAGS=tag1,tag2
make create FILE=photo2.jpg TAGS=tag2,tag3
```

**Update a blob:**
```bash
make update ID=<uuid> FILE=photo3.jpg TAGS=tag2,another_tag
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