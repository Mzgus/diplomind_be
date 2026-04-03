set dotenv-load := true

stop:
  docker compose stop

stop_and_wipe:
  docker compose down && docker volume rm diplomind

reboot:  
  just stop_and_wipe && just start

start:
  docker compose up -d

db_up:
  docker compose up -d db

restart:
  docker compose start

seed:
  docker exec -i db psql -U diplomind_u -d diplomind_db -f /seed/seed.sql

test_query:
  docker compose exec db psql -U ${POSTGRES_USER} -d ${POSTGRES_DB} -c "SELECT * FROM users_sheets;"

psql:
  docker compose exec db psql -U ${POSTGRES_USER} -d ${POSTGRES_DB}

seed:
  cat seed/seed.sql | docker compose exec -T db psql -U ${POSTGRES_USER} -d ${POSTGRES_DB}

uninstall:
  docker compose down --rmi all && docker volume rm diplomind

sudo_stop:
  sudo docker compose stop

sudo_stop_and_wipe:
  sudo docker compose down && sudo docker volume rm diplomind

sudo_reboot:  
  just sudo_stop_and_wipe && just sudo_start

sudo_start:
  sudo docker compose up -d

sudo_db_up:
  sudo docker compose up -d db

sudo_restart:
  sudo docker compose start

sudo_seed:
  sudo docker exec -i db psql -U diplomind_u -d diplomind_db -f /seed/seed.sql

sudo_test_query:
  sudo docker compose exec db psql -U ${POSTGRES_USER} -d ${POSTGRES_DB} -c "SELECT * FROM users_sheets;"

sudo_psql:
  sudo docker compose exec db psql -U ${POSTGRES_USER} -d ${POSTGRES_DB}

sudo_seed:
  cat seed/seed.sql | sudo docker compose exec -T db psql -U ${POSTGRES_USER} -d ${POSTGRES_DB}

sudo_uninstall:
  sudo docker compose down --rmi all && sudo docker volume rm diplomind