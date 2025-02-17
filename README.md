<!--
SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>

SPDX-License-Identifier: CC0-1.0
-->

# *Proj*ect *Var*iables

[![License: GPL-3.0-or-later](
    https://img.shields.io/badge/License-GPL%203.0+-blue.svg)](
    https://www.gnu.org/licenses/gpl-3.0.html)
[![REUSE status](
    https://api.reuse.software/badge/github.com/hoijui/projvar)](
    https://api.reuse.software/info/github.com/hoijui/projvar)
<!--
[![crates.io](
    https://img.shields.io/crates/v/projvar.svg)](
    https://crates.io/crates/projvar)
-->
[![Docs](
    https://docs.rs/projvar/badge.svg)](
    https://docs.rs/projvar)
[![dependency status](
    https://deps.rs/repo/github/hoijui/projvar/status.svg)](
    https://deps.rs/repo/github/hoijui/projvar)
[![Build status](
    https://github.com/hoijui/projvar/workflows/build/badge.svg)](
    https://github.com/hoijui/projvar/actions)

This tool tries to extract a certain small set
of project related properties,
using various sources.

## Example scenario

In your CI job:

1. Check out your repo
2. Run this tool (`projvar`)
   * it ensures a a few properties are known, for example:
     * `PROJECT_NAME="My Project"`
     * `PROJECT_VERSION="my-proj-1.2.3-44-ge73gf28"`
     * `PROJECT_REPO_WEB_URL="https://github.com/user/my-proj"`
   * it stores them somehow, typically into a file,
     using [BASH `source`](
     https://opensource.com/article/20/6/bash-source-command) compatible syntax
3. Run some other tool that uses these environment variables.
   For example, you may include it in a QRCode,
   which you then print onto your project.

## How to compile

You need to install Rust(lang) and Cargo.

Then get the whole repo plus git submodules with:

```bash
git clone --recurse-submodules https://github.com/hoijui/projvar.git
cd projvar
```

Then you can run:

```bash
scripts/build
```

If all goeas well, the executable can be found at `target/release/projvar`.

## Get the tool

As for now, you have two choices:

1. [Compile it](#how-to-compile) yourself
1. Download a Linux x86\_64 staticially linked binary from
   [the releases page](https://github.com/hoijui/projvar/releases)

## Usage

```bash
$ projvar --help
projvar 0.5.1

Robin Vobruba <hoijui.quaero@gmail.com>

    A tiny CLI tool that tries to gather project specific meta-data in different ways,
    to store them into environment variables in a file
    for later use by other tools.
    See --list for the keys set by this tool.


USAGE:
    osh [OPTIONS]

OPTIONS:
    -a, --all
            Marks all properties as required. See --none,--fail,--require,--require-not.

    -A, --show-all-retrieved <MD-FILE>...
            Shows a table (in Markdown syntax) of all properties and the values retrieved for each
            from each individual source. Writes to log(Info), if no target file is given as
            argument.

    -C, --project-root <DIR>
            The root directory of the project, mainly used for SCM (e.g. git) information gathering.
            [default: .]

    -d, --dry
            Set Whether to skip the actual setting of environment variables.

    -D, --variable <KEY=VALUE>...
            A key-value pair (aka a variable) to be used as input, as it it was specified as an
            environment variable. Value provided with this take precedense over environment
            variables - they overwrite them. See -I,--variable-file for supplying a lot of such
            pairs at once.

    -e, --env-out
            Write resulting values directy into the environment

    -f, --fail
            Fail if no value is available for any of the required properties (see
            --all,--none,--require,--require-not)

    -F, --log-level <log-level>...
            Set the log-level [possible values: None, Errors, Warnings, Info, Debug, Trace]

    -h, --help
            Print help information

    -I, --variables-file <FILE>...
            An input file containing KEY=VALUE pairs, one per line (BASH style). Empty lines, and
            those starting with "#" or "//" are ignored. See -D,--variable for specifying one pair
            at a time.

        --key-prefix <STRING>
            The key prefix to be used when writing out values in the sinks. For example "PROJECT_"
            -> "PROJECT_VERSION", "PROJECT_NAME", ... [default: PROJECT_]

    -l, --list
            Prints a list of all the environment variables that are potentially set by this tool
            onto stdout and exits.

    -L, --log-file <log-file>
            Writes a detailed log to the specifed file.

    -n, --none
            Marks all properties as *not* required. See --all,--fail,--require,--require-not.

    -N, --require-not <KEY>...
            A key of a variable whose value is *not* required. For example PROJECT_NAME (see --list
            for all possible keys). Can be used either on the base of the default requried list or
            all (see --fail,--all,--none,--require)

    -o, --overwrite <overwrite>
            Whether to overwrite already set values in the output. [default: All] [possible values:
            All, None, Main, Alternative]

    -O, --file-out <FILE>...
            Write evaluated values into a file, one KEY-VALUE pair per line (BASH syntax). Note that
            "-" has no special meaning here; it does not mean stdout, but rather the file "./-".
            [default: .projvars.env.txt]

        --only-required
            Only fetch and output the required values (see --all,--none,--require, --require-not).

    -P, --show-primary-retrieved <MD-FILE>...
            Shows a list (in Markdown syntax) of all properties and the primary values retrieved for
            each, accumulated over the sources. Writes to log(Info), if no target file is given as
            argument.

    -q, --quiet
            Supresses all log-output to stdout, and only shows errors on stderr (see -L,--log-level
            to also disable those). This does not affect the log level for the log-file.

    -R, --require <KEY>...
            Mark a propery as required. You may use the property name (e.g. "Name") or the variable
            key (e.g. "PROJECT_NAME"); See --list for all possible keys. If at least one such option
            is present, the default required values list is cleared (see
            --fail,--all,--none,--require-not).

    -t, --hosting-type <hosting-type>
            As usually most kinds of repo URL property values are derived from the clone URL, it is
            essential to know how to construct them. Different hosting softwares construct them
            differently. By default, we try to derive it from the clone URL domain, but if this is
            not possible, this switch allows to set the hosting software manually. [default:
            Unknown] [possible values: GitHub, GitLab, BitBucket, SourceHut, Gitea, Girocco,
            RocketGit, Allura, Unknown]

    -T, --date-format <date-format>
            Date format string for generated (vs supplied) dates. For details, see
            https://docs.rs/chrono/latest/chrono/format/strftime/index.html [default: "%Y-%m-%d
            %H:%M:%S"]

    -v, --verbose
            More verbose log output; useful for debugging. See -L,--log-level for more fine-graine
            control.

    -V, --version
            Print version information

    -x, --no-env-in
            Disable the use of environment variables as input
```

The list of all supported keys/properties (as shown by `--list`):

| Default Required | Key | Description |
| - | --- | ------------ |
| [ ] | `PROJECT_BUILD_ARCH` | The computer hardware architecture we are building on. (common values: 'x86', 'x86_64') |
| [ ] | `PROJECT_BUILD_BRANCH` | The development branch name, for example: "master", "develop" |
| [ ] | `PROJECT_BUILD_DATE` | Date of this build, for example: "2021-12-31 23:59:59" (see --date-format) |
| [ ] | `PROJECT_BUILD_HOSTING_URL` | Web URL under which the generated output will be available, for example: https://osegermany.gitlab.io/OHS-3105 |
| [ ] | `PROJECT_BUILD_NUMBER` | The build number (1, 2, 3) starts at 1 for each repo and branch. |
| [ ] | `PROJECT_BUILD_OS` | The operating system we are building on. (common values: 'linux', 'macos', 'windows') |
| [ ] | `PROJECT_BUILD_OS_FAMILY` | The operating system family we are building on. (should be either 'unix' or 'windows') |
| [ ] | `PROJECT_BUILD_TAG` | The tag of a commit that kicked off the build. This value is only available on tags. Not available for builds against branches. |
| [ ] | `PROJECT_CI` | 'true' if running on a CI/build-bot; unset otherwise. |
| [x] | `PROJECT_LICENSE` | The main License identifier of the sources, prefferably from the SPDX specs, for example: "AGPL-3.0-or-later", "CC-BY-SA-4.0" |
| [x] | `PROJECT_LICENSES` | The identifiers of all the licenses of this project, prefferably from the SPDX specs, comma separated, for example: "AGPL-3.0-or-later, CC0-1.0, Unlicense" |
| [x] | `PROJECT_NAME` | The human focused name of the project. |
| [x] | `PROJECT_NAME_MACHINE_READABLE` | The machine readable name of the project. |
| [x] | `PROJECT_REPO_CLONE_URL` | The repo clone URL, HTTP(S) version. This is commonly used for anonymous fetch-only access. |
| [ ] | `PROJECT_REPO_CLONE_URL_SSH` | The repo clone URL, SSH version. This is commonly used for authenticated, fetch and push access. |
| [x] | `PROJECT_REPO_COMMIT_PREFIX_URL` | The repo commit prefix URL. Add commit SHA. The part in []: [https://github.com/hoijui/nim-ci/commit]/23f84b91] |
| [x] | `PROJECT_REPO_ISSUES_URL` | The repo issues URL, for example: https://gitlab.com/openflexure/openflexure-microscope/issues |
| [x] | `PROJECT_REPO_RAW_VERSIONED_PREFIX_URL` | The repo raw prefix URL. Add version (tag, branch, SHA) and file path. The part in []: [https://raw.githubusercontent.com/hoijui/nim-ci]/master/.github/workflows/docker.yml] |
| [x] | `PROJECT_REPO_VERSIONED_DIR_PREFIX_URL` | The repo directory prefix URL. Add version (tag, branch, SHA) and directory path. The part in []: [https://github.com/hoijui/nim-ci]/master/.github/workflows/docker.yml] |
| [x] | `PROJECT_REPO_VERSIONED_FILE_PREFIX_URL` | The repo file prefix URL. Add version (tag, branch, SHA) and file path. The part in []: [https://github.com/hoijui/nim-ci]/master/.github/workflows/docker.yml] |
| [x] | `PROJECT_REPO_WEB_URL` | The repo web UI URL, for example: https://gitlab.com/OSEGermany/OHS-3105 |
| [x] | `PROJECT_VERSION` | The project version, for example: "1.10.3", "0.2.0-1-ga5387ac-dirty" |
| [x] | `PROJECT_VERSION_DATE` | Date this version was committed to source control, for example: "2021-12-31 23:59:59" (see --date-format) |
