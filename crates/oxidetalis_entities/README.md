# Oxidetalis database entities

This crate contains the database entities for the Oxidetalis homeserver, using
SeaORM.

## Must to know
- Don't import sea_orm things in another crates, import the entities and sea_orm
  things from this crate, from `prelude` module.

## How to write a new entity
Check the [SeaORM
documentation](https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-structure/)
for more information about how to write entities.

## License
This crate is licensed under the MIT license.
