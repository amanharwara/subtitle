# subtitle

Find & download subtitles from the terminal.

Installation:

```
cargo install subtitle
```

Usage:

```
subtitle 1.0.0
Find & download subtitles from the terminal.

USAGE:
    subtitle.exe [FLAGS] [OPTIONS] [FILE(S)]...

ARGS:
    <FILE(S)>...    Get subtitles for a file or multiple files.

FLAGS:
    -d, --dont-use-lang-fallback    When used, app will stop if subtitles aren't found for the
                                    selected language.
    -h, --help                      Prints help information
    -V, --version                   Prints version information

OPTIONS:
        --api <API_NAME>     The API to use for downloading subtitles. [default: subdb] [possible
                             values: subdb, opensubtitles]
    -l, --lang <LANGUAGE>    The preferred language to download the subtitles in. [default: en]
```
