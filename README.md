# file_server

`http` file server written in rust using [tokio](https://tokio.rs/) and
[hyper](https://hyper.rs/).

## Create a config

A JSON configuration file is required to run `file_server`.

Configuration schema:

```
{
  "host": <string>,
  "port": <number>,
  "directory": <string>
}
```

Change the `host` property to serve from a specific host.

Change the `port` property to serve from a different port.

Change the `directory` property to target an alternative directory. The `directory` property can be an absolute or relative path. A relative path is relative to the location of the JSON configuration file.

A valid configuration example can be found at
`./file_server.example.json`

## Install file_server

Execute the following to install `file_server`.

```
git clone https://github.com/herebythere/file_server
cargo install --path file_server/file_server
```

## Run file_server

The `file_server` application accepts one argument from the command line:

- A valid `file_server` JSON configuration file

```
file_server <path_to_configuration_file>
```

Execute the following to host the `./demo` directory using `file_server`.

```
file_server file_server/v0.1/file_server.example.json
```

Open a browser and visit `http://localhost:3000`.

## Licence

BSD 3-Clause License
