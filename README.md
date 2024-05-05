# Database To Anything

DBTA is a lightweight database schema migration and version control tool with a DSL for autogenerating code w/ a centralized source of truth. <br>
DBTA is still in-progress. Probably don't build anything with it that needs to last more than week :)

# Commands

Alter log.sql's schema. <br>
`$ dbta -u log.sql -s "SQL Command"`

Generate code from template and schema. <br>
`$ dbta -g template.gen -f log.sql`

Display schema. <br>
`$ dbta -d log.sql`

# SqlGen Documentation

## General Functions

`[Function Name](param_1, param_2, ...)["String", {}, {}, {}]`

## COLUMN

```
[COLUMN i](table, separator, exclude){"Column: {}", i}
```

Examples:

```
Non-FK Columns:
[COLUMN i](users, "\n", "*_id"){"* {}", i.name}
```

```
[COLUMN i](users, exclude = "*"){"This should never really appear", i.name}
```
