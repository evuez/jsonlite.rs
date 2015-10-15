# JSONlite (based on [nodesocket/jsonlite](https://github.com/nodesocket/jsonlite))

##### A simple, self-contained, serverless, zero-configuration, [json](http://www.json.org/) document store.

JSONlite sandboxes the current working directory just like [SQLite](https://www.sqlite.org/). The data directory is named `jsonlite.data`, and each json document is saved pretty printed as a uuid.

## Proof of Concept

JSONlite is a proof of concept, and it may not make any sense to actually use in development or production.

## Installation

```shell
git clone https://github.com/evuez/jsonlite.rs.git
cd jsonlite.rs
cargo build --release
ln -s $PWD/target/release/jsonlite /usr/local/bin/jsonlite
```

## Requirements

1. Rust

## Configuration

Not available for now.

## API

### set

> set \<json\> - Writes a json document and returns the document id

````shell
➜ jsonlite set "{\"name\":\"John Doe\",\"active\":true,\"permissions\":{\"read\":true,\"write\":false}}"
666B81D6-3F8A-4D57-BA3F-11FA8FC47246
````

### get

> get \<document-id\> - Retrieves a json document by document id

````shell
➜ jsonlite get 666B81D6-3F8A-4D57-BA3F-11FA8FC47246
{
    "active": true,
    "name": "John Doe",
    "permissions": {
        "read": true,
        "write": false
    }
}
````

### delete

> delete \<document-id\> - Deletes a json document by document id

````shell
➜ jsonlite delete 666B81D6-3F8A-4D57-BA3F-11FA8FC47246
````

### drop

> drop - Drops the database

````shell
➜ jsonlite drop
Are you sure you want to drop '/jsonlite.data'? (y/n) y
````

### version

> version - Displays the current version

````shell
➜ jsonlite version
0.1.0
````

#### help

> help - Displays help

````
➜ jsonlite help
Usage: jsonlite <cmd> <data>
       jsonlite <cmd>
````

## Support, Bugs, And Feature Requests

Create issues here in GitHub (https://github.com/evuez/jsonlite.rs/issues).

## Versioning

For transparency and insight into the release cycle, and for striving to maintain backward compatibility, JSONlite will be maintained under the semantic versioning guidelines.

Releases will be numbered with the follow format:

`<major>.<minor>.<patch>`

And constructed with the following guidelines:

+ Breaking backward compatibility bumps the major (and resets the minor and patch)
+ New additions without breaking backward compatibility bumps the minor (and resets the patch)
+ Bug fixes and misc changes bumps the patch

For more information on semantic versioning, visit http://semver.org/.
