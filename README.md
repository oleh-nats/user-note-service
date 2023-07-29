cargo add diesel dotenv actix --features "diesel/postgres diesel/r2d2 diesel/chrono"
cargo install diesel_cli --no-default-features --features postgres
cargo install diesel_cli_ext
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo -i -u postgres
createdb postgres
psql
ALTER USER postgres WITH PASSWORD '1'; # type your password here
\q
exit
setup .env file, copy from .env.example
for testing use postman or thunder client

cargo r -r
use requests from thunder-collection_backend_test.json or yours

