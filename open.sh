#!/usr/bin/env bash
set -euf -o pipefail

TEMPLATE="./template.rs"
CODE_DIR="./codes"

default="unamed-$(date -Iseconds)"
name="${1:-$default}"

function filepath() {
    echo "${CODE_DIR}/${1:-$name}.rs"
}

function discard() {
    rm "$(filepath)"
    exit 1
}

if [[ -f "$(filepath)" ]]; then
    "${EDITOR:-vi}" "$(filepath)"
    exit 0
fi

mkdir -p "$CODE_DIR"
cp "$TEMPLATE" "$(filepath)"
"${EDITOR:-vi}" "$(filepath)"

while true; do
    echo "Do you wish to save this as '$(filepath)'?"
    read -p "[YES/no/rename] > " ans
    a="${ans:0:1}"
    case "${a^^}" in
       '' ) ;&
        Y ) break;;
        N ) discard ;;
        R ) old_path="$(filepath)"
            read -p "New name? (save as $(filepath [new-name])) > " name
            mv "$old_path" "$(filepath)" ;;
        * ) ;;
    esac
done
echo "Saved as '$(filepath)'."
