# This justfile is for the contrbutors of this project, not for the end user.
#
# Requirements for this justfile:
# - Linux distribution
# - just (Of course) <https://github.com/casey/just>
# - cargo (For the build and tests) <https://doc.rust-lang.org/cargo/getting-started/installation.html>

set shell := ["/usr/bin/env", "bash", "-c"]

JUST_EXECUTABLE := "just -u -f " + justfile()
header := "Available tasks:\n"
# Get the MSRV from the Cargo.toml
msrv := `cat Cargo.toml | grep "rust-version" | sed 's/.*"\(.*\)".*/\1/'`


_default:
    @{{JUST_EXECUTABLE}} --list-heading "{{header}}" --list

# Run the CI
@ci: && msrv
    cargo build --all-targets
    cargo fmt --all -- --check
    cargo clippy --all-targets -- -D warnings

# Check that the current MSRV is correct
@msrv:
    rustup toolchain install {{msrv}}
    echo "Checking MSRV ({{msrv}})"
    cargo +{{msrv}} check -q --workspace
    echo "MSRV is correct"

run:
    docker-compose up -d db
    RUST_LOG=debug cargo run -p oxidetalis -- --config config.toml

# Generate the contributors list from the git log
# then add it to the CONTRIBUTORS file
contributors:
    #! /usr/bin/env bash
    maintainers="- Awiteb <a@4rs.nl>\n- Amjad Alsharafi <me@amjad.alsharafi.dev>"
    reviewers=`git log --pretty=format:"%b" | grep -i "Reviewed-by" | sed 's/Reviewed-by: /- /' | sort | uniq`
    reporters=`git log --pretty=format:"%b" | grep -i "Reported-by" | sed 's/Reported-by: /- /' | sort | uniq`
    suggesters=`git log --pretty=format:"%b" | grep -i "Suggested-by" | sed 's/Suggested-by: /- /' | sort | uniq`
    helpers=`git log --pretty=format:"%b" | grep -i "Helped-by" | sed 's/Helped-by: /- /' | sort | uniq`
    authors=`git log --pretty=format:"- %an <%ae>" | sort | uniq | grep -v "actions"`
    co_authors=`git log --pretty=format:"%b" | grep -i "Co-authored-by" | sed 's/Co-authored-by: /- /' | sort | uniq`
    contributors_count=`echo -e "${maintainers}\n${authors}\n${co_authors}\n${reviewers}\n${reporters}\n${suggesters}\n${helpers}" | grep -Ev '^$' | sort | uniq | wc -l`
    footer="Oxidetalis Homeserver currently has ${contributors_count} contributors!\n\nInterested in becoming a contributor? Read the [CONTRIBUTING.md](CONTRIBUTING.md) file to get started."

    file="# Contributors\nA heartfelt thank you to all the contributors who have helped make Oxidetalis Homeserver better. Below is a list of the contributors who have played a crucial role in the development and improvement of Oxidetalis Homeserver.\n\n## Maintainers\nMaintainers are responsible for overseeing Oxidetalis, keeping it up to date, and ensuring it runs smoothly.\n\n${maintainers}"
    if [ -n "$authors" ]; then
        file="${file}\n\n## Code Authors\nCode authors have written the code for Oxidetalis.\n\n${authors}"
    fi
    if [ -n "$co_authors" ]; then
        file="${file}\n${co_authors}"
    fi
    if [ -n "$reviewers" ]; then
        file="${file}\n\n## Reviewers\nCode reviewers have reviewed the code and provided feedback, suggestions, and improvements.\n\n${reviewers}"
    fi
    if [ -n "$reporters" ]; then
        file="${file}\n\n## Bug Reporters\nBug reporters have identified and reported bugs in Oxidetalis, helping to improve its quality.\n\n${reporters}"
    fi
    if [ -n "$suggesters" ]; then
        file="${file}\n\n## Feature Suggesters\nFeature suggesters have proposed new features to enhance Oxidetalis.\n\n${suggesters}"
    fi
    if [ -n "$helpers" ]; then
        file="${file}\n\n## Helpers\nHelpers have contributed in various ways that do not involve writing code, such as providing feedback, suggestions, and improvements.\n\n${helpers}"
    fi
    file="${file}\n\n---\n${footer}"
    echo -e "${file}" | fmt -sw 80 > "CONTRIBUTORS.md"

[private]
alias r := run
