#!/usr/bin/env bash
set -euf -o pipefail

TEMPLATE="./template.rs"
CODE_DIR="./codes"
DEFAULT_FILENAME="unamed-$(date -Iseconds)"

# 0: disable git feature
# 1: enable git feature with independent commits
#    (make a independent commit for each change)
# 2: enable git feature with amendment
#    (amend the last commit if it has changes for the same file)
AUTO_GIT=2

name="${1:-$DEFAULT_FILENAME}"

function filepath() {
    echo "${CODE_DIR}/${1:-$name}.rs"
}

function git_update() {
    # do nothing if the git feature is disabled
    [[ ${AUTO_GIT} -ne 0 ]] || return

    local file="$1" message="$2"
    if (git log --pretty=oneline -1 | grep "$message"); then
        git add "$file" && git commit --amend --no-edit
    else
        git add "$file" && git commit -m "$message"
    fi
}

if [[ -f "$(filepath)" ]]; then
    "${EDITOR:-vi}" "$(filepath)"
    git_update "$(filepath)" "Update $(filepath)"
    exit 0
fi

mkdir -p "$(dirname "$(filepath)")"
cp "$TEMPLATE" "$(filepath)"
"${EDITOR:-vi}" "$(filepath)"

while true; do
    echo "Do you wish to save this as '$(filepath)'?"
    read -rp "[YES/no/rename] > " ans
    a="${ans:0:1}"
    case "${a^^}" in
        '' | Y ) break;;
        N ) : Discard changes
            rm "$(filepath)"
            exit 1 ;;
        R ) : Rename the file
            old_path="$(filepath)"
            read -rp "New name? (save as $(filepath '[new-name]')) > " name
            mv "$old_path" "$(filepath)" ;;
        * ) : Retry ;;
    esac
done

echo "Saved as '$(filepath)'."
git_update "$(filepath)" "Add $(filepath)"
