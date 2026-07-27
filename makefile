ifneq (,$(wildcard .env))
    include .env
    export
endif

# if your migrate error generated temp in this project 
cornucopia-temp:
	$env:TEMP="$(PWD)\temp"; $env:TMP="$(PWD)\temp"; make cornucopia


migrate:
	refinery migrate -e DATABASE_URL -p ./database/migrations

cornucopia:
	cornucopia live ${DATABASE_URL} -q ./database/sql -d ./src/db

build:
	cargo build

update:
	cargo update

run:
	cargo run
