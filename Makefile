BINARY := latex-compiler
TARGET := x86_64-unknown-linux-musl
COMMIT := $(shell git rev-parse --short HEAD)

.PHONY:
	@echo "No default target."

build:
	cargo zigbuild --target $(TARGET) --release -p $(BINARY)

build-image: build
	mkdir -p ./docker/tmp/
	cp ./target/$(TARGET)/release/$(BINARY) ./docker/tmp/$(BINARY)
	chmod +x ./docker/tmp/$(BINARY)
	docker build -t "latex_compiler:latest" ./docker
	docker build -t "latex_compiler:$(COMMIT)" ./docker

run: build-image
	docker run -p 3000:3000 latex_compiler

stop:
	docker rm $(docker stop $(docker ps -a --filter ancestor=latex_compiler --format="{{.ID}}"))

publish-image: build-image
	docker tag latex_compiler:$(COMMIT) ghcr.io/sendcv/latex_compiler:$(COMMIT)
	docker tag latex_compiler:$(COMMIT) ghcr.io/sendcv/latex_compiler:latest
	docker push ghcr.io/sendcv/latex_compiler:$(COMMIT)
	docker push ghcr.io/sendcv/latex_compiler:latest

current-tag:
	@echo $(COMMIT)
