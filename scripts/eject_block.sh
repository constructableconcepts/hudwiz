#!/bin/bash

# This script ejects a component block from the blocks directory
# into the main components directory for customization.

# Color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if a block name is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: No block name provided.${NC}"
    echo "Usage: ./eject_block.sh <BlockName>"
    exit 1
fi

BLOCK_NAME=$1
SOURCE_DIR="hudwiz/frontend/src/components/blocks"
TARGET_DIR="hudwiz/frontend/src/components"
SOURCE_FILE="${SOURCE_DIR}/${BLOCK_NAME}.rs"

# Check if the source file exists
if [ ! -f "$SOURCE_FILE" ]; then
    echo -e "${RED}Error: Block '${BLOCK_NAME}' not found at ${SOURCE_FILE}${NC}"
    exit 1
fi

TARGET_FILE="${TARGET_DIR}/${BLOCK_NAME}.rs"

# Check if the file already exists in the target directory
if [ -f "$TARGET_FILE" ]; {
    echo -e "${RED}Error: Component '${BLOCK_NAME}' already exists at ${TARGET_FILE}${NC}"
    exit 1
}

# Copy the file
cp "$SOURCE_FILE" "$TARGET_FILE"

echo -e "${GREEN}Success! Block '${BLOCK_NAME}' ejected to ${TARGET_FILE}${NC}"
echo "You may now modify it and integrate it into your application."