# cecho

A command-line tool that makes it easier to print colorized text:

```bash
cecho 'My {cyan: hovercraft} is {red underline: full of eels}!'
```

## Installation

TBD

## Options

- `-n` - Suppress the trailing newline
- `-h, --help` - Show help message with all formatting options and examples

## Usage

```bash
# Basic usage
cecho 'Hello world!'

# Colored text
cecho 'This is {red: red text}'
cecho 'Status: {green: SUCCESS}'

# Styled text
cecho 'This is {bold: important}'
cecho 'This is {italic: emphasized}'
cecho 'This is {underline: underlined}'

# Combined formatting
cecho 'This is {red bold: very important}'
cecho 'Warning: {yellow bold underline: critical issue}'

# Multiple formatted sections
cecho 'Hello {cyan: world}, this is {red: amazing}!'
cecho 'Status: {green: PASSED} - {blue: 42 tests} completed'

# Log-style output
cecho '[{green: INFO}] Application started'
cecho '[{yellow: WARN}] Connection timeout'
cecho '[{red: ERROR}] {red bold: Critical failure}'

# Suppress newline
cecho -n 'Loading{cyan: ...}'
```

## Formatting Options

### Colors
- `black`, `blue`, `cyan`, `green`, `magenta`, `red`, `white`, `yellow`

### Styles
- `blink`, `bold`, `dim`, `italic`, `reverse`, `strikethrough`, `underline`

### Combining Formats
Multiple formats can be combined with spaces:
- `{red bold: text}` - Red and bold
- `{blue underline: text}` - Blue and underlined
- `{yellow bold underline: text}` - Yellow, bold, and underlined

## Help

For a complete list of options, formatting choices, and examples, run:

```bash
cecho -h
# or
cecho --help
```
