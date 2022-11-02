# `clockify-transfer`

Transfer Jira timesheet CSV exports into Clockify.

```
Usage: clockify-transfer [OPTIONS] <FILE>
       clockify-transfer <COMMAND>

Commands:
  config-template  Print a config template
  init             Create a config file. Defaults to: $XDG_CONFIG/clockify-transfer/config.yml
  help             Print this message or the help of the given subcommand(s)

Arguments:
  <FILE>  The Jira timesheet CSV export file. Use '-' to read from stdin

Options:
  -d, --dry-run        Output what would happen, but don't actually submit to Clockify
  -c, --config <FILE>  Load configuration from a custom location. Defaults to: $XDG_CONFIG/clockify-transfer/config.yml
  -h, --help           Print help information
```

## Installation

```bash
$ cargo install --git https://github.com/marcaddeo/clockify-transfer
```

## Configuration

`clockify-transfer` uses a YAML configuration file located at
`$XDG_CONFIG/clockify-transfer/config.yml`. 

You can create a configuration file by running `clockify-transfer init`. This
will create a configuration file that looks something like this:

```yml
# The Clockify API base path, with a trailing slash.
#
# Default value: "https://api.clockify.me/api/v1/"
#api_base_path: "https://api.clockify.me/api/v1/"

# Your Clockify API key.
#
# Required! This value must be specified.
#api_key:

# Your Clockify Workspace Name.
#
# Required! This value must be specified.
#workspace:

# A mapping of Jira Project Key to Clockify Project Names.
#
# Example:
#
# project_map:
#   PROJ: Project Name Goes Here
#   ANOTHER: Another Project Name Goes Here
#
# Required! This value must be specified.
#project_map:
```

You can get your API key by going to: https://app.clockify.me/user/settings
