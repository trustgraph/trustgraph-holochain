#!/usr/bin/env bash
set -euo pipefail
# set -x


export CARGO_HOME=$(pwd)/.cargo

main () {
  run_targets $*
}

run_targets () {
  for func in $*; do
    run $func
  done
}

run () {
  func=$1
  if [[ $(type -t $func) == function ]]; then
    $func
  else
    echo "Unknown command: ${func}"
    exit 1
  fi
}

shell () {
  if [[ -z $1 ]]; then
    echo "Error: \`shell\` called with no arguments"
    exit 1
  fi

  if [[ -n ${2:-} ]]; then
    echo "Error: \`shell\` called with more than one argument: $*"
    exit 1
  fi

  command=$(echo $1 | tr '\n' ' ' | tr -s ' ') # remove newlines, collapse spaces
  echo "----> ${command}"
  eval "$command"
}

in_test_env () {
  # shell "CARGO_TARGET_DIR=target"
  run_targets $*
}

# targets to be passed in as command line args:

checks () {
  time bin/run.sh test clippy
}

test_watch () {
  shell "cargo watch -- bin/run.sh test"
}

test_watch_clear () {
  shell "cargo watch --clear -- bin/run.sh test"
}

test () {
  in_test_env build_dna test_metal
}

test_metal () {
  shell "time cargo test -- --nocapture"
}

dev_watch () {
	shell "cargo watch -- bin/run.sh dev"
}

dev_watch_clear () {
	shell "cargo watch --clear -- bin/run.sh dev"
}

dev () {
  build_happ

	shell "
    hc sandbox generate
      --run 8000
      --app-id holonexus
      --root tmp
      workdir/happ/holonexus.happ
    "
}

package_ci () {
  shell "cd ../ui"
  shell "npm ci"
  shell "cd -"
  run package
}

package () {
  run build_happ
  run package_ui
  shell "hc web-app pack workdir"
}

package_ui() {
  shell "cd ../ui"
  shell "npm run build || true"  # TODO fix and then remove `|| true`
  shell "npm run zip"
  shell "cd -"
}

build () {
  run build_happ
}

build_happ () {
  run build_dna
  shell "hc app pack workdir/happ"
}

build_dna () {
  run build_zome
  shell "hc dna pack workdir/dna"
}

build_zome () {
  # CARGO_TARGET_DIR=target/zome
  # cargo build --target wasm32-unknown-unknown
  shell "cargo build --release --target wasm32-unknown-unknown"
}

clippy () {
  # CARGO_TARGET_DIR=target/clippy
  shell "cargo clippy --all-targets --all-features -- -D warnings"
}

clippy_watch () {
  shell "cargo watch --clear -- bin/run.sh clippy"
}

clean () {
  shell "git clean -Xfd"
  shell "cargo clean"
}

shipit () {
  if [[ -z $(git status --porcelain) ]]; then
    run checks
    shell "git push origin HEAD"
  else
    echo "Error: git status not clean:"
    set -x
    git status
    exit 1
  fi
}

main $*
