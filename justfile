stop_and_wipe:
  sudo docker compose down && sudo docker volume rm diplomind

reboot:  
  just stop_and_wipe && just start

start:
  sudo docker compose up -d

test_query:
  sudo docker compose exec db psql -U diplomind_u -d diplomind_db -c "SELECT   \* FROM users_sheets;"

psql:
  sudo docker compose exec db psql -U diplomind_u -d diplomind_db

