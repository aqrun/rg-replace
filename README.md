cargo run -- --pattern s

cargo run -- --pattern "sea_orm_migration::prelude::\\*;" --replace-with "use sea_orm_migration::prelude::123::*"  --search-path /Users/aqrun/workspace/github.com/aqrun/oicnp/migration --confirm  
