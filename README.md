### dotnetdetect
> :sparkles: Small utility to detect version of .NET Framework installed on a Windows system.

[![Build status](https://ci.appveyor.com/api/projects/status/r1jjy8w5xg74soho?svg=true)](https://ci.appveyor.com/project/stpettersens/dotnetdetect)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/stpettersens/dotnetdetect/blob/master/LICENSE)

#### Building from source

Prerequisties to build are:
* [Rust tools](https://www.rust-lang.org) (rustc, cargo, etc).

* [Ruby](https://www.ruby-lang.org), [Rake](https://ruby.github.io/rake/), [gems](https://rubygems.org/pages/download) and [OS gem](https://rubygems.org/gems/os)
or  [Rake in Rust](https://github.com/stpettersens/rakeinrust) (optional).

* [UPX](https://upx.github.io) (optional).

Building:

* `> rake` or: `cargo build --release`
* `> rake upx` # if you want to compress resultant executable with UPX.

#### Usage
```
Usage: dotnetdetect.exe [-h | -v][-r <dotnet version>][-q]

Options are:

-r | --required: The required minimal .NET version.
-q | --quiet: Do not output version to stdout; just exit code (0 for pass; -1 for fail).
-p | --part-quiet: Output version to stdout, but not "Meets..." message.
-h | --help: Display this usage information and exit.
-v | --version: Display program version information and exit.
```
