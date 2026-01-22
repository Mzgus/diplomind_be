stop_and_wipe:
  docker compose down && docker volume rm diplomind

reboot:  
  just stop_and_wipe && just start

start:
  docker compose up -d

test_query:
  docker compose exec db psql -U diplomind_u -d diplomind_db -c "SELECT   \* FROM users_sheets;"

psql:
  docker compose exec db psql -U diplomind_u -d diplomind_db

sudo_stop_and_wipe:
  sudo docker compose down && sudo docker volume rm diplomind

sudo_reboot:  
  just sudo_stop_and_wipe && just sudo_start

sudo_start:
  sudo docker compose up -d

sudo_test_query:
  sudo docker compose exec db psql -U diplomind_u -d diplomind_db -c "SELECT   \* FROM users_sheets;"

sudo_psql:
  sudo docker compose exec db psql -U diplomind_u -d diplomind_db