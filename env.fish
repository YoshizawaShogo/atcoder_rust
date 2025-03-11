set SCRIPT_DIR (realpath (dirname (status --current-filename)))
export CARGO_ATCODER_TEST_CONFIG_DIR="$SCRIPT_DIR/template"
mkdir -p "$SCRIPT_DIR/contests"
# cp "$SCRIPT_DIR/template/rust-toolchain" "$SCRIPT_DIR/contests"

function atcoder_new --argument-names contest_name
    set contest_dir "$SCRIPT_DIR/contests/$contest_name"
    if test -d "$contest_dir"
        echo "$contest_dir already exists."
        return 0
    end

    pushd "$SCRIPT_DIR/contests"
    cargo atcoder new "$contest_name"
    popd

    cp -r "$SCRIPT_DIR/template/.vscode" "$contest_dir"
    # cp -r "$SCRIPT_DIR/template/rust-toolchain" "$contest_dir"

    code "$contest_dir"
end
