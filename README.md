## Postgres

brew install postgresql@16
brew services start postgresql@16
brew info postgresql@16
echo 'export PATH="/opt/homebrew/opt/postgresql@16/bin:$PATH"' >> ~/.zshrc

Install the sqlx CLI

cargo install sqlx-cli --no-default-features --features rustls,postgres

create the DB:

sqlx database create

To create a new migration:

sqlx migrate add <migration name>
