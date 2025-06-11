#!/bin/bash

set -e

# If only there was a tool that would make it easier to output colorized strings...
CYAN='\033[0;36m'
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Formatting "${CYAN}./src/*${YELLOW}" ...${NC}"

if ! command -v rustfmt &> /dev/null; then
    echo -e "${RED}Error: rustfmt is not installed or not in PATH${NC}"
    echo "Please install rustfmt with: rustup component add rustfmt"
    exit 1
fi

if [ ! -d "./src" ]; then
    echo -e "${RED}Error: ./src directory not found${NC}"
    exit 1
fi

rust_files=$(find ./src -name "*.rs" -type f)

if [ -z "$rust_files" ]; then
    echo -e "${YELLOW}No Rust files found in ./src directory${NC}"
    exit 0
fi

formatted_count=0
for file in $rust_files; do
    if rustfmt "$file"; then
        ((formatted_count++))
    else
        echo -e "${RED}Failed to format $file${NC}"
    fi
done

echo -e "${GREEN}Formatted $formatted_count Rust file(s)${NC}"
