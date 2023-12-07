## Postgres

brew install postgresql@16
brew services start postgresql@16
brew info postgresql@16
echo 'export PATH="/opt/homebrew/opt/postgresql@16/bin:$PATH"' >> ~/.zshrc
createdb todo-axum-htmx
