mod common;

use common::{Workspace, typstyle_cmd_snapshot};

#[test]
fn test_one() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a  =  0");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let a = 0

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_one_inplace() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a  =  0");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "-i"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
}

#[test]
fn test_one_quiet() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a  =  0");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "-q"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let a = 0

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_typstyle_toml_function_hint() {
    let mut space = Workspace::new();
    space.write(
        "typstyle.toml",
        r#"
[function-hints.my-table]
kind = "table"
columns = 2
"#,
    );
    space.write_tracked("a.typ", "#my-table([1], [2], [3], [4])");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #my-table(
      [1], [2],
      [3], [4],
    )

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_typstyle_toml_format_config_from_parent_dir() {
    let mut space = Workspace::new();
    space.write(
        "typstyle.toml",
        r#"
indent_width = 4
"#,
    );
    space.write_tracked(
        "nested/a.typ",
        "#let f(x) = {\nfor i in range(0, 5) {\n     x = x + i\n }\n}",
    );

    typstyle_cmd_snapshot!(space.cli().args(["nested/a.typ"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let f(x) = {
        for i in range(0, 5) {
            x = x + i
        }
    }

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_typstyle_toml_format_config_for_stdin() {
    let space = Workspace::new();
    space.write(
        "typstyle.toml",
        r#"
wrap_text = true
max_width = 20
"#,
    );
    let stdin = "lorem  ipsum   dolor sit amet, consectetur   adipiscing elit.";

    typstyle_cmd_snapshot!(space.cli().pass_stdin(stdin), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    lorem ipsum dolor
    sit amet,
    consectetur
    adipiscing elit.

    ----- stderr -----
    ");
}

#[test]
fn test_cli_overrides_typstyle_toml_format_config() {
    let space = Workspace::new();
    space.write(
        "typstyle.toml",
        r#"
wrap_text = true
max_width = 20
"#,
    );
    let stdin = "lorem  ipsum   dolor sit amet, consectetur   adipiscing elit.";

    typstyle_cmd_snapshot!(space.cli().args(["--line-width=80"]).pass_stdin(stdin), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    lorem ipsum dolor sit amet, consectetur adipiscing elit.

    ----- stderr -----
    ");
}

#[test]
fn test_typstyle_toml_qualified_function_hint() {
    let mut space = Workspace::new();
    space.write(
        "typstyle.toml",
        r#"
[function-hints."pkg.my-table"]
kind = "table"
columns = 2
"#,
    );
    space.write_tracked("a.typ", "#pkg.my-table([1], [2], [3], [4])");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #pkg.my-table(
      [1], [2],
      [3], [4],
    )

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_typstyle_toml_invalid_function_hint() {
    let mut space = Workspace::new();
    space.write(
        "typstyle.toml",
        r#"
[function-hints.my-table]
kind = "unknown"
"#,
    );
    space.write_tracked("a.typ", "#my-table([1], [2], [3], [4])");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ"]), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
      Cause: failed to parse [TEMP_PATH]/project/typstyle.toml
      Cause: function-hints.my-table.kind must be `table` or `grid`
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_typstyle_toml_invalid_format_config() {
    let mut space = Workspace::new();
    space.write(
        "typstyle.toml",
        r#"
max_width = "wide"
"#,
    );
    space.write_tracked("a.typ", "#let a  =  0");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ"]), @r"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
      Cause: failed to parse [TEMP_PATH]/project/typstyle.toml
      Cause: max_width must be an integer
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_one_erroneous() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let
    ----- stderr -----
    warn: Failed to parse a.typ. The source is erroneous.
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_one_inplace_erroneous() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "-i"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    warn: Failed to parse a.typ. The source is erroneous.
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_one_check_quiet() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a  =  0");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "--check", "-q"]), @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_two_0() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a = 0\n");
    space.write_tracked("b.typ", "#let b = 1\n");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "b.typ"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let a = 0
    #let b = 1

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_two_1() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a = 0\n");
    space.write_tracked("b.typ", "#let b  =  1\n");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "b.typ"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let a = 0
    #let b = 1

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_two_2() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a  =  0\n");
    space.write_tracked("b.typ", "#let b  =  1\n");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "b.typ"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let a = 0
    #let b = 1

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_two_0_inplace() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a = 0\n");
    space.write_tracked("b.typ", "#let b = 1\n");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "b.typ", "-i"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    assert!(space.is_unmodified("a.typ"));
    assert!(space.is_unmodified("b.typ"));
}

#[test]
fn test_two_1_inplace() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a = 0\n");
    space.write_tracked("b.typ", "#let b  =  1\n");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "b.typ", "-i"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    assert!(space.is_unmodified("a.typ"));
    assert_eq!(space.read_string("b.typ"), "#let b = 1\n");
}

#[test]
fn test_two_2_inplace() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a  =  0\n");
    space.write_tracked("b.typ", "#let b  =  1\n");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "b.typ", "-i"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    assert_eq!(space.read_string("a.typ"), "#let a = 0\n");
    assert_eq!(space.read_string("b.typ"), "#let b = 1\n");
}

#[test]
fn test_two_0_check() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a = 0\n");
    space.write_tracked("b.typ", "#let b = 1\n");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "b.typ", "--check"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_two_1_check() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a = 0\n");
    space.write_tracked("b.typ", "#let b  =  1\n");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "b.typ", "--check"]), @r"
    success: false
    exit_code: 1
    ----- stdout -----
    Would reformat: b.typ

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_two_2_check() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a  =  0\n");
    space.write_tracked("b.typ", "#let b  =  1\n");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "b.typ", "--check"]), @r"
    success: false
    exit_code: 1
    ----- stdout -----
    Would reformat: a.typ
    Would reformat: b.typ

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_cwd() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a = 0\n");
    space.write_tracked("b.typ", "#let b  =  1\n");
    space.write_tracked("d/c.typ", "#let c  =  2\n");

    typstyle_cmd_snapshot!(space.cli().args(["./d/.."]), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let a = 0
    #let b = 1
    #let c = 2

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_cwd_check() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a = 0\n");
    space.write_tracked("b.typ", "#let b  =  1\n");
    space.write_tracked("d/c.typ", "#let c  =  2\n");

    typstyle_cmd_snapshot!(space.cli().args(["./d/..", "--check"]), @r"
    success: false
    exit_code: 1
    ----- stdout -----
    Would reformat: b.typ
    Would reformat: d/c.typ

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_many() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a = 0\n");
    space.write_tracked("b.typ", "#let b  =  1\n");
    space.write_tracked("d/c.typ", "#let c  =  2\n");
    space.write_tracked("d/d/e.typ", "#let d  =  3\n");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "b.typ", "d"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----
    #let a = 0
    #let b = 1
    #let c = 2
    #let d = 3

    ----- stderr -----
    ");

    assert!(space.is_unmodified("a.typ"));
}

#[test]
fn test_many_inplace() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a = 0\n");
    space.write_tracked("b.typ", "#let b  =  1\n");
    space.write_tracked("d/c.typ", "#let c  =  2\n");
    space.write_tracked("d/d/e.typ", "#let d  =  3\n");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "b.typ", "d", "-i"]), @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    assert!(space.is_unmodified("a.typ"));
    assert!(space.is_modified("b.typ"));
    assert!(space.is_modified("d/c.typ"));
    assert!(space.is_modified("d/d/e.typ"));
}

#[test]
fn test_many_check() {
    let mut space = Workspace::new();
    space.write_tracked("a.typ", "#let a = 0\n");
    space.write_tracked("b.typ", "#let b  =  1\n");
    space.write_tracked("d/c.typ", "#let c  =  2\n");
    space.write_tracked("d/d/e.typ", "#let d  =  3\n");

    typstyle_cmd_snapshot!(space.cli().args(["a.typ", "b.typ", "d", "--check"]), @r"
    success: false
    exit_code: 1
    ----- stdout -----
    Would reformat: b.typ
    Would reformat: d/c.typ
    Would reformat: d/d/e.typ

    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_diff_single_line() {
    let mut space = Workspace::new();
    space.write_tracked("single.typ", "#let x=1+2");

    typstyle_cmd_snapshot!(space.cli().args(["single.typ", "--diff"]), @r"
    success: false
    exit_code: 1
    ----- stdout -----
    --- single.typ
    +++ single.typ
    @@ -1 +1 @@
    -#let x=1+2
    \ No newline at end of file
    +#let x = 1 + 2


    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}

#[test]
fn test_diff_multiline() {
    let mut space = Workspace::new();
    space.write_tracked("multi.typ", "#let x=1\n#let y=2+3");

    typstyle_cmd_snapshot!(space.cli().args(["multi.typ", "--diff"]), @r"
    success: false
    exit_code: 1
    ----- stdout -----
    --- multi.typ
    +++ multi.typ
    @@ -1,2 +1,2 @@
    -#let x=1
    -#let y=2+3
    \ No newline at end of file
    +#let x = 1
    +#let y = 2 + 3


    ----- stderr -----
    ");

    assert!(space.all_unmodified());
}
