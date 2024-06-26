# Oxidetalis configurations
A library for managing configurations of Oxidetalis homeserver.

## Key Features
- **Load and write configurations**: Load configurations from a file and write
  configurations to a file.
- **Multiple configuration entries**: The configurations are collected from CLI
  arguments, environment variables, and configuration files.
- **Configuration validation**: Validate the configurations before using them.
- **Configuration defaults**: Set default values for configurations.

## Must to know
- The configurations are loaded in the following order (from highest priority to
  lowest priority)
  1. Command-line options
  2. Environment variables
  3. Configuration file
  4. Default values (or ask you to provide the value)
- The configurations are written to the configuration file every time you run
  the server, even if you don't change any configuration. This is to ensure that
  the configuration file is always up-to-date.


## License
This crate is licensed under the MIT license.
