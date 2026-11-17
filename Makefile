.PHONY: up down logs clean create get list update delete

# client parameters, passed as `make create FILE=... TAGS=a,b`
FILE ?=
TAGS ?=
ID ?=
OUT ?=
NAME ?=
CONTENT_TYPE ?=
LIMIT ?= 10
OWNER_ID ?=
ACCESS_LEVEL ?=
METADATA ?=

up:
	docker compose up --build -d

down:
	docker compose down

logs:
	docker compose logs -f blob-service

clean:
	docker compose down -v

create:
	@test -n "$(FILE)" || { echo "usage: make create FILE=path [TAGS=a,b] [METADATA=k=v] [OWNER_ID=id] [ACCESS_LEVEL=level]"; exit 1; }
	cargo run --quiet --bin client -- create --file $(FILE) \
		$(if $(TAGS),--tags $(TAGS)) \
		$(if $(METADATA),--metadata $(METADATA)) \
		$(if $(OWNER_ID),--owner-id $(OWNER_ID)) \
		$(if $(ACCESS_LEVEL),--access-level $(ACCESS_LEVEL))

get:
	@test -n "$(ID)" || { echo "usage: make get ID=uuid [OUT=path]"; exit 1; }
	cargo run --quiet --bin client -- get --id $(ID) $(if $(OUT),--out $(OUT))

list:
	cargo run --quiet --bin client -- list \
		$(if $(NAME),--name $(NAME)) \
		$(if $(CONTENT_TYPE),--content-type $(CONTENT_TYPE)) \
		$(if $(TAGS),--tags $(TAGS)) \
		--limit $(LIMIT)

update:
	@test -n "$(ID)" || { echo "usage: make update ID=uuid [FILE=path] [NAME=name] [TAGS=a,b] [METADATA=k=v] [CONTENT_TYPE=type]"; exit 1; }
	@test -n "$(FILE)$(NAME)$(TAGS)$(METADATA)$(CONTENT_TYPE)" || { echo "nothing to update"; exit 1; }
	cargo run --quiet --bin client -- update --id $(ID) \
		$(if $(FILE),--file $(FILE)) \
		$(if $(NAME),--name $(NAME)) \
		$(if $(CONTENT_TYPE),--content-type $(CONTENT_TYPE)) \
		$(if $(TAGS),--tags $(TAGS)) \
		$(if $(METADATA),--metadata $(METADATA))

delete:
	@test -n "$(ID)" || { echo "usage: make delete ID=uuid"; exit 1; }
	cargo run --quiet --bin client -- delete --id $(ID)
