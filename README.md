# A Rust version of [JSONlite](https://github.com/nodesocket/jsonlite)

## JSONlite

*A simple, self-contained, serverless, zero-configuration, [json](http://www.json.org/) document store.*

JSONlite sandboxes the current working directory just like [SQLite](https://www.sqlite.org/). The data directory is named `jsonlite.data`, and each json document is saved pretty printed as a uuid.

## Note

This is my first Rust code so it's far from being good code. I'll accept most PRs as long as they don't break API compatibility :).

## Installation

```shell
git clone https://github.com/evuez/jsonlite.rs.git
cd jsonlite.rs
cargo build --release
ln -s $PWD/target/release/jsonlite /usr/local/bin/jsonlite
```

## API

See [original API](https://github.com/nodesocket/jsonlite#api).

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
