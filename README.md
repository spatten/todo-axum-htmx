## Postgres
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fspatten%2Ftodo-axum-htmx.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fspatten%2Ftodo-axum-htmx?ref=badge_shield)


brew install postgresql@16
brew services start postgresql@16
brew info postgresql@16
echo 'export PATH="/opt/homebrew/opt/postgresql@16/bin:$PATH"' >> ~/.zshrc

Install the sqlx CLI

cargo install sqlx-cli --no-default-features --features rustls,postgres

create the DB:

sqlx database create

Run the migrations:

sqlx migrate run

To create a new migration:

sqlx migrate add <migration name>


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fspatten%2Ftodo-axum-htmx.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fspatten%2Ftodo-axum-htmx?ref=badge_large)