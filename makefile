clean-db:
	docker kill gandalf-db
	docker rm gandalf-db
	docker volume rm gandalf_postgres_data
	docker compose up -d
