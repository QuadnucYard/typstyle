/// This module provides functionality to format Typst files either in-place or by checking
/// their formatting via standard input/output.
///
/// Adapted from: https://github.com/astral-sh/ruff/blob/main/crates/ruff_linter/src/fs.rs
use std::{
    io::Read,
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::{Context, Result, bail};
use itertools::Itertools;
use log::{debug, error, info, warn};
use toml::Value;
use typst_syntax::Source;
use typstyle_core::{Config, FormatFacts, FunctionHint, Typstyle, format_ast};
use walkdir::{DirEntry, WalkDir};

use crate::{
    ExitStatus,
    cli::{CliArguments, DebugArgs, StyleArgs},
    diff::SourceDiff,
    fs,
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum FormatMode {
    /// Write the formatted contents back to the file.
    Write,
    /// Check if the file is formatted, but do not write the formatted contents back.
    Check,
    /// Show unified diff of what formatting changes would be made.
    Diff,
}

impl FormatMode {
    pub(crate) fn from_cli(cli: &CliArguments) -> Self {
        if cli.check {
            FormatMode::Check
        } else if cli.diff {
            FormatMode::Diff
        } else {
            FormatMode::Write
        }
    }
}

impl StyleArgs {
    pub fn apply_to_config(&self, mut config: Config) -> Config {
        if let Some(line_width) = self.line_width {
            config.max_width = line_width;
        }
        if let Some(indent_width) = self.indent_width {
            config.tab_spaces = indent_width;
        }
        if self.no_reorder_import_items {
            config.reorder_import_items = false;
        }
        if self.wrap_text {
            config.wrap_text = true;
            config.collapse_markup_spaces = true;
        }
        config
    }
}

pub fn format_stdin(args: &CliArguments) -> Result<ExitStatus> {
    let typstyle = make_typstyle(args, None)?;

    format_one(None, &typstyle, args).map(|res| match res {
        FormatResult::Formatted(_) if args.check || args.diff => ExitStatus::Failure,
        _ => ExitStatus::Success,
    })
}

pub fn format(args: &CliArguments) -> Result<ExitStatus> {
    #[derive(Default)]
    struct Summary {
        format_count: usize,
        unchanged_count: usize,
        error_count: usize,
    }
    let mut summary = Summary::default();

    let mode = FormatMode::from_cli(args);
    let paths = resolve_typst_files(&args.input);
    if paths.is_empty() {
        warn!("No Typst files found under the given path(s).");
        return Ok(ExitStatus::Success);
    }

    let start_time = Instant::now();
    for file in paths {
        let typstyle = make_typstyle(args, Some(&file))?;
        let res = format_one(Some(&file), &typstyle, args).unwrap_or_else(|e| {
            error!("{e}");
            summary.error_count += 1;
            FormatResult::Erroneous
        });

        // Check if the content is already well-formatted (unchanged)
        match res {
            FormatResult::Formatted(_) => summary.format_count += 1,
            _ => summary.unchanged_count += 1,
        }
    }
    let duration = start_time.elapsed();

    fn num_files(num: usize) -> String {
        if num > 1 {
            format!("{num} files")
        } else {
            format!("{num} file")
        }
    }

    match mode {
        FormatMode::Write => debug!(
            "Successfully formatted {} ({} unchanged) in {:?}",
            num_files(summary.format_count),
            summary.unchanged_count,
            duration
        ),
        FormatMode::Check => debug!(
            "{} would be reformatted ({} already formatted), checked in {:?}",
            num_files(summary.format_count),
            summary.unchanged_count,
            duration
        ),
        FormatMode::Diff => debug!(
            "{} would be reformatted ({} already formatted), checked with diff in {:?}",
            num_files(summary.format_count),
            summary.unchanged_count,
            duration
        ),
    }
    if summary.error_count > 0 {
        // Syntax errors are not counted here.
        bail!(
            "failed to format {} due to IO error",
            num_files(summary.error_count)
        );
    }

    Ok(match mode {
        FormatMode::Check | FormatMode::Diff if summary.format_count > 0 => ExitStatus::Failure,
        _ => ExitStatus::Success,
    })
}

fn make_typstyle(args: &CliArguments, input: Option<&Path>) -> Result<Typstyle> {
    let project_config = load_typstyle_toml_config(input)?;
    let config = args.style.apply_to_config(project_config.config);
    Ok(Typstyle::new(config).with_experimental_facts(project_config.facts))
}

/// Formats a single `.typ` file or input from stdin.
///
/// This function formats the file provided as an argument, or reads from stdin if no file is given.
/// If in-place formatting is requested, it overwrites the file with the formatted content.
///
/// # Parameters
/// - `input`: An optional path to a `.typ` file to be formatted. If `None`, input is read from stdin.
/// - `args`: CLI arguments.
///
/// # Returns
/// - `Ok(FormatStatus::Changed)` if the file was reformatted.
/// - `Ok(FormatStatus::Unchanged)` if the file was unchanged or contained errors.
/// - `Err` if reading from or writing to the file fails.
fn format_one(
    input: Option<&Path>,
    typstyle: &Typstyle,
    args: &CliArguments,
) -> Result<FormatResult> {
    let use_stdout = !args.inplace && !args.check && !args.diff;
    let unformatted = get_input(input)?;

    let res = format_debug(&unformatted, typstyle, &args.debug);
    match &res {
        FormatResult::Formatted(res) => {
            if args.inplace {
                // We have already validated that the input is Some.
                write_back(input.unwrap(), res)?;
            } else if args.check {
                if let Some(path) = input {
                    info!("Would reformat: {}", fs::relativize_path(path));
                } else {
                    // For stdin, we don't output anything in check mode
                    // just rely on the exit code
                }
            } else if args.diff {
                print_unified_diff(&unformatted, res, input);
            } else {
                print!("{res}");
            }
        }
        FormatResult::Unchanged => {
            if use_stdout {
                print!("{unformatted}");
            }
        }
        FormatResult::Erroneous => {
            if use_stdout {
                print!("{unformatted}"); // still prints the original content to enable piping
            }
            if let Some(path) = input {
                warn!(
                    "Failed to parse {}. The source is erroneous.",
                    fs::relativize_path(path)
                );
            } else {
                warn!("Failed to parse stdin. The source is erroneous.");
            }
        }
    }
    Ok(res)
}

enum FormatResult {
    Formatted(String),
    Unchanged,
    Erroneous,
}

fn format_debug(content: &str, typstyle: &Typstyle, args: &DebugArgs) -> FormatResult {
    let source = Source::detached(content);
    let root = source.root();
    if args.ast {
        println!("{}", format_ast(root));
    }

    let start_time = Instant::now();
    let f = typstyle.format_source(source);
    if args.pretty_doc {
        match f.render_ir() {
            Ok(ir) => println!("{ir}"),
            Err(e) => error!("Failed to render IR: {e}"),
        }
    }
    let Ok(res) = f.render() else {
        return FormatResult::Erroneous;
    };

    if args.timing {
        println!("Formatting completed in {:?}", start_time.elapsed());
    }

    // Compare `res` with `content` to perform CI checks
    if res != content {
        FormatResult::Formatted(res)
    } else {
        FormatResult::Unchanged
    }
}

fn get_input(input: Option<&Path>) -> Result<String> {
    match input {
        Some(path) => std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display())),
        None => {
            let mut buffer = String::new();
            std::io::stdin()
                .read_to_string(&mut buffer)
                .with_context(|| "failed to read from stdin")?;
            Ok(buffer)
        }
    }
}

fn write_back(path: &Path, content: &str) -> Result<()> {
    std::fs::write(path, content)
        .with_context(|| format!("failed to write to the file {}", path.display()))
}

#[derive(Debug, Clone, Default)]
struct ProjectConfig {
    config: Config,
    facts: FormatFacts,
}

fn load_typstyle_toml_config(input: Option<&Path>) -> Result<ProjectConfig> {
    let Some(config_path) = find_typstyle_toml(input)? else {
        return Ok(ProjectConfig::default());
    };
    let content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read {}", config_path.display()))?;
    parse_typstyle_toml_config(&content)
        .with_context(|| format!("failed to parse {}", config_path.display()))
}

fn find_typstyle_toml(input: Option<&Path>) -> Result<Option<PathBuf>> {
    let start = match input.and_then(Path::parent) {
        Some(parent) => parent.to_path_buf(),
        None => std::env::current_dir().context("failed to get current directory")?,
    };

    for dir in start.ancestors() {
        let candidate = dir.join("typstyle.toml");
        if candidate.is_file() {
            return Ok(Some(candidate));
        }
    }
    Ok(None)
}

fn parse_typstyle_toml_config(content: &str) -> Result<ProjectConfig> {
    let root = content.parse::<Value>()?;
    let config = parse_format_config(&root)?;
    let facts = parse_format_facts(&root)?;

    Ok(ProjectConfig { config, facts })
}

fn parse_format_config(root: &Value) -> Result<Config> {
    let mut config = Config::default();

    if let Some(max_width) = get_usize(
        root,
        &["max_width", "max-width", "line_width", "line-width"],
    )? {
        config.max_width = max_width;
    }
    if let Some(tab_spaces) = get_usize(
        root,
        &["tab_spaces", "tab-spaces", "indent_width", "indent-width"],
    )? {
        config.tab_spaces = tab_spaces;
    }
    if let Some(blank_lines_upper_bound) = get_usize(
        root,
        &["blank_lines_upper_bound", "blank-lines-upper-bound"],
    )? {
        config.blank_lines_upper_bound = blank_lines_upper_bound;
    }
    if let Some(collapse_markup_spaces) =
        get_bool(root, &["collapse_markup_spaces", "collapse-markup-spaces"])?
    {
        config.collapse_markup_spaces = collapse_markup_spaces;
    }
    if let Some(reorder_import_items) =
        get_bool(root, &["reorder_import_items", "reorder-import-items"])?
    {
        config.reorder_import_items = reorder_import_items;
    }
    if let Some(wrap_text) = get_bool(root, &["wrap_text", "wrap-text"])? {
        config.wrap_text = wrap_text;
        config.collapse_markup_spaces |= wrap_text;
    }

    Ok(config)
}

fn parse_format_facts(root: &Value) -> Result<FormatFacts> {
    let mut facts = FormatFacts::default();

    let Some(function_hints) = root.get("function-hints").and_then(Value::as_table) else {
        return Ok(facts);
    };

    for (callee_name, entry) in function_hints {
        let kind = entry
            .get("kind")
            .and_then(Value::as_str)
            .with_context(|| format!("function-hints.{callee_name}.kind must be a string"))?;
        let columns = entry
            .get("columns")
            .and_then(Value::as_integer)
            .map(|columns| {
                usize::try_from(columns).with_context(|| {
                    format!("function-hints.{callee_name}.columns must be non-negative")
                })
            })
            .transpose()?;

        let hint = match kind {
            "table" => FunctionHint::table_like(columns),
            "grid" => FunctionHint::grid_like(columns),
            _ => bail!("function-hints.{callee_name}.kind must be `table` or `grid`"),
        };
        facts.add_function_name_hint(callee_name, hint);
    }

    Ok(facts)
}

fn get_usize(root: &Value, keys: &[&str]) -> Result<Option<usize>> {
    let Some((key, value)) = get_unique(root, keys)? else {
        return Ok(None);
    };
    let Some(value) = value.as_integer() else {
        bail!("{key} must be an integer");
    };
    Ok(Some(
        usize::try_from(value).with_context(|| format!("{key} must be non-negative"))?,
    ))
}

fn get_bool(root: &Value, keys: &[&str]) -> Result<Option<bool>> {
    let Some((key, value)) = get_unique(root, keys)? else {
        return Ok(None);
    };
    value
        .as_bool()
        .with_context(|| format!("{key} must be a boolean"))
        .map(Some)
}

fn get_unique<'a, 'k>(
    root: &'a Value,
    keys: &'k [&'k str],
) -> Result<Option<(&'k str, &'a Value)>> {
    let mut found = keys
        .iter()
        .filter_map(|key| root.get(*key).map(|value| (*key, value)));
    let first = found.next();
    if let Some((first_key, _)) = first
        && let Some((second_key, _)) = found.next()
    {
        bail!("{first_key} conflicts with {second_key}");
    }
    Ok(first)
}

fn resolve_typst_files(input: &[PathBuf]) -> Vec<PathBuf> {
    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .is_some_and(|s| s.starts_with('.'))
    }

    let mut files = Vec::new();
    let mut has_dir = false;
    for path in input.iter().map(fs::normalize_path).unique() {
        if path.is_dir() {
            has_dir = true;
            let entries = WalkDir::new(path)
                .into_iter()
                .filter_entry(|e| !is_hidden(e))
                .filter_map(Result::ok);
            for entry in entries {
                if entry.file_type().is_file() && entry.path().extension() == Some("typ".as_ref()) {
                    files.push(entry.into_path());
                }
            }
        } else {
            files.push(path.clone());
        }
    }
    if has_dir {
        files.sort_unstable();
    }
    files
}

fn print_unified_diff(original: &str, modified: &str, path: Option<&Path>) {
    print!(
        "{}",
        SourceDiff {
            original,
            modified,
            path,
        }
    );
}
