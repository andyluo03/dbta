# Database To Anything

DBTA is a lightweight database schema migration and version control tool with a DSL for autogenerating code w/ a centralized source of truth. <br>
DBTA is still in-progress. Probably don't build anything with it that needs to last more than week :).

# Commands

Alter log.sql's schema.
`$ dbta -u log.sql -s "SQL Command"`

Generate code from template and schema.
`$ dbta -g template.gen -f log.sql`

Display schema.
`$ dbta -d log.sql`
