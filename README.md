# Install sea orm cli
`cargo install sea-orm-cli@1.0.0-rc.5`

# Generate migration directory for schema workspace to `migration/src`
`sea-orm-cli migrate init`

# Generate a new table schema to the migration schema workspace -note consider deleting migration code in the main when deleting migration file
`sea-orm-cli migrate generate table_name`

# Write and modify the db schema then migrate -note when creating relations, table order matters
`sea-orm-cli migrate fresh`

# Generate entity files of database `actixdb` to `src/internal/entity` with serializer and deserializer
`sea-orm-cli generate entity --with-serde both -o src/internal/entity`

# CLI Docs help
`sea-orm-cli -h`