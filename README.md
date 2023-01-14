# tarsum

Compute a checksum for each file in a Tar archive.

This is a rewrite of [tarsum.py](https://www.guyrutenberg.com/2009/04/29/tarsum-02-a-read-only-version-of-tarsum/) in Rust.

## Installing

```
cargo install --git https://github.com/guyru/tarsum
```

## Usage

```
tarsum [OPTIONS] [INPUT]

Arguments:
  [INPUT]  File to process. If missing or set as -, read stdin

Options:
  -o, --output <OUTPUT>      Save signatures to file
  -c, --checksum <CHECKSUM>  Select a checksum algorithm (md5, sha1, sha256, sha384, sha512)
                             [default: sha256]
  -z, --zero                 Output a zero byte (ASCII NUL) at the end of each line, rather than a
                             newline. This option enables other programs to parse the output even
                             when that output would contain data with embedded newlines. Also file
                             name escaping is not used
  -h, --help                 Print help information
  -V, --version              Print version information

```

## License

Copyright (C) 2022  Guy Rutenberg

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.

## Authors
- Author: [Guy Rutenberg](https://www.guyrutenberg.com)
