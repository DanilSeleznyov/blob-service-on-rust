.PHONY: up down logs clean create get list update delete

# client parameters, passed as `make create FILE=... TAGS=a,b`
FILE ?=
TAGS ?=
ID ?=
OUT ?=
NAME ?=
CONTENT_TYPE ?=
LIMIT ?= 10

up:
	docker compose up --build -d

down:
	docker compose down

logs:
	docker compose logs -f blob-service

clean:
	docker compose down -v

create:
	@test -n "$(FILE)" || { echo "usage: make create FILE=path [TAGS=a,b]"; exit 1; }
	cargo run --quiet --bin client -- create --file $(FILE) $(if $(TAGS),--tags $(TAGS))

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
	@test -n "$(ID)" && test -n "$(FILE)" || { echo "usage: make update ID=uuid FILE=path [TAGS=a,b]"; exit 1; }
	cargo run --quiet --bin client -- update --id $(ID) --file $(FILE) $(if $(TAGS),--tags $(TAGS))

delete:
	@test -n "$(ID)" || { echo "usage: make delete ID=uuid"; exit 1; }
	cargo run --quiet --bin client -- delete --id $(ID)
