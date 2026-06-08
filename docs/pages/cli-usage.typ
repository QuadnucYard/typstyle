#import "./book.typ": *

#show: book-page.with(title: "CLI Usage")

#raw(read("../assets/generated/cli-help.txt"), block: true)

= Basic Usage

```bash
# Format multiple files
typstyle -i chapter1.typ chapter2.typ appendix.typ

# Format entire project
typstyle -i .

# Format with specific configuration
typstyle -l 100 -t 4 --wrap-text -i src/
```

= Arguments

== Input Files

```bash
# Format from stdin (default)
cat file.typ | typstyle

# Format specific files
typstyle file1.typ file2.typ

# Format directories (recursively)
typstyle src/ docs/
```

= Main Options

== Format Control

=== In-Place Formatting

```bash
# Modify files directly
typstyle -i file.typ
typstyle --inplace file.typ
```

=== Check Mode

```bash
# Exit with non-zero if formatting needed
typstyle --check src/
```

```bash
# Like --check, but shows unified diff of what formatting changes would be made
typstyle --diff src/
```

== Format Configuration

=== Line Width

```bash
# Set maximum line width (default: 80)
typstyle -l 100 file.typ
typstyle --line-width 100 file.typ
```

=== Indentation

```bash
# Set indentation width (default: 2)
typstyle -t 4 file.typ
typstyle --indent-width 4 file.typ
```

=== Text Wrapping

```bash
# Wrap text in markup to fit line width
typstyle --wrap-text file.typ
```

== Configuration File

Typstyle looks for `typstyle.toml` in the current directory for stdin, or in the formatted file's directory and its ancestors for file inputs. Command-line options override values from `typstyle.toml`.

```toml
max_width = 100
indent_width = 4
wrap_text = true
reorder_import_items = false

[function-hints.my-table]
kind = "table"
columns = 2
```

Supported formatter keys are:

- `max_width` or `line_width`
- `tab_spaces` or `indent_width`
- `blank_lines_upper_bound`
- `collapse_markup_spaces`
- `reorder_import_items`
- `wrap_text`

Keys may also use kebab case, such as `max-width` or `wrap-text`.

`function-hints` is experimental. It can mark wrapper functions as `table` or `grid` so they use table/grid formatting when no LSP semantic facts are available.

= Debug Options

== AST Output

```bash
# Print the Abstract Syntax Tree
typstyle -a file.typ
typstyle --ast file.typ
```

== Pretty Document Output

```bash
# Print the internal pretty document representation
typstyle -p file.typ
typstyle --pretty-doc file.typ
```

== Timing Information

```bash
# Show elapsed time taken by the formatter
typstyle --timing file.typ
```

= Logging Options

See CLI help.
