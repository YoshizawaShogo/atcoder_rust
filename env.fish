set SCRIPT_DIR (realpath (dirname (status --current-filename)))
export CARGO_ATCODER_TEST_CONFIG_DIR="$SCRIPT_DIR/template"
mkdir -p "$SCRIPT_DIR/contests"

function atcoder --argument-names contest_name
    pushd "$SCRIPT_DIR/contests"
    cargo atcoder new "$contest_name"
    popd

    set contest_dir "$SCRIPT_DIR/contests/$contest_name"
    cp -r "$SCRIPT_DIR/template/.vscode" "$contest_dir"
    cp -r "$SCRIPT_DIR/template/rust-toolchain" "$contest_dir"
    code "$contest_dir"
end
