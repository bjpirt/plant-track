.PHONY: env
env:
	docker compose --project-name plant-track -f environment/docker-compose.yml up -d --wait
