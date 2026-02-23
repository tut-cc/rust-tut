#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0

"""generate_rust_analyzer - Generates the rust-project.json file for rust-analyzer.

This program is heavily inspired by
https://github.com/Rust-for-Linux/linux/blob/18b7491480025420896e0c8b73c98475c3806c6f/scripts/generate_rust_analyzer.py

Some editor plugins do not play well with the nightly Rust toolchain when the
unstable `-Z script` feature is used. Hopefully this workaround becomes
unnecessary when editor integration with nightly smoothes out or the standalone
script feature becomes available in the stable Rust toolchain.

Tracking Issue for cargo-script RFC 3424 #12207
https://github.com/rust-lang/cargo/issues/12207

Rustlings <6.0 implements rust-project.json schema, but they seem to include
exercise files only and not include standard library as dependencies
https://github.com/rust-lang/rustlings/blob/9a743f80c57cc6bf27819589a8ddb5a5579ab1a4/src/project.rs

Although rust-project.json uses "sysroot-src" for both sysroot source and
sysroot toolchain, in Nix environment they are actually located in two separate
derivations.
"""

import argparse
import json
import pathlib
from pathlib import Path
import sys


def generate_crates(sysroot_src: Path, srctree: Path) -> list[dict]:
    # Fill the crates list -- dependencies need to come first.
    #
    # Avoid O(n^2) iterations by keeping a map of indices.
    crates: list[dict] = []
    crates_indices: dict[str, int] = {}

    def append_crate(
        display_name: str,
        root_module: Path,
        deps: list[str],
        cfg=[],
        is_workspace_member=True,
        is_proc_macro=False,
    ):
        crates_indices[display_name] = len(crates)
        crates.append(
            {
                "display_name": display_name,
                "root_module": str(root_module),
                "edition": "2024",
                "is_workspace_member": is_workspace_member,
                "is_proc_macro": is_proc_macro,
                "deps": [{"crate": crates_indices[dep], "name": dep} for dep in deps],
                "cfg": cfg,
            }
        )

    # First, the ones in `rust/` since they are a bit special.
    # Note that there might be some missing dependencies because I haven't
    # checked if these list cover all parts of standard library in Rust.
    append_crate(
        "core",
        sysroot_src / "core" / "src" / "lib.rs",
        [],
        is_workspace_member=False,
    )
    append_crate(
        "compiler_builtins",
        sysroot_src / "compiler-builtins" / "compiler-builtins" / "src" / "lib.rs",
        [],
    )
    append_crate(
        "alloc",
        sysroot_src / "alloc" / "src" / "lib.rs",
        ["core", "compiler_builtins"],
        is_workspace_member=False,
    )
    append_crate(
        "std",
        sysroot_src / "std" / "src" / "lib.rs",
        ["alloc"],
        is_workspace_member=False,
    )

    for path in srctree.glob("*.rs"):
        append_crate(path.stem, path, ["core", "alloc", "std"])

    for path in srctree.glob("solutions/*.rs"):
        append_crate(f"solutions:{path.stem}", path, ["core", "alloc", "std"])

    return crates


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--verbose", "-v", action="store_true")
    parser.add_argument("sysroot_toolchain", type=pathlib.Path)  # rustc --print sysroot
    parser.add_argument("rust_src_path", type=pathlib.Path)  # $RUST_SRC_PATH
    parser.add_argument("srctree", type=pathlib.Path)  # $PROJECT_ROOT/l1_basics
    args = parser.parse_args()

    rust_project = {
        "crates": generate_crates(args.rust_src_path, args.srctree),
        "sysroot_src": str(args.sysroot_toolchain),
    }

    json.dump(rust_project, sys.stdout, sort_keys=True, indent=4)


if __name__ == "__main__":
    main()
