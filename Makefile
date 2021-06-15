IMAGE=gcr.io/ml-team-307117/indeed-scrapper

ifeq ($(IMAGE_TAG),)
	IMAGE_TAG=0.1.0
endif

docker/indeed-scrapper:
	cargo build --release
	mv target/release/indeed-scrapper docker/indeed-scrapper

docker/image: docker/indeed-scrapper
	docker build docker -f docker/Dockerfile -t $(IMAGE):$(IMAGE_TAG)
	touch docker/image

docker/push: docker/image
	docker push $(IMAGE):$(IMAGE_TAG)
	touch docker/push

docker/push-latest: docker/image
	docker tag $(IMAGE):$(IMAGE_TAG) $(IMAGE):latest
	docker push $(IMAGE):latest
	touch docker/push-latest

clean:
	cargo clean
	rm -rf docker/indeed-scrapper
	rm -rf docker/image
	rm -rf docker/push-latest
	rm -rf docker/push