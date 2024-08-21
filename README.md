# Install sea orm cli
`cargo install sea-orm-cli@1.0.0-rc.5`

# Generate migration schema workspace to `migration/src`
`sea-orm-cli migrate init`

# Generate a new table schema to the migration schema workspace
`sea-orm-cli migrate generate create_table*name`

# Write and modify the db schema structure then migrate
`sea-orm-cli migrate fresh`

# Generate entity files of database `actixdb` to `src/internal/entity`
`sea-orm-cli generate entity -o src/internal/entity`

# CLI Docs help
`sea-orm-cli -h`