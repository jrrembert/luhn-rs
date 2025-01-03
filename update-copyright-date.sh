#!/bin/bash

# Check if file is provided as argument
if [ $# -ne 1 ]; then
    echo "Usage: $0 <markdown_file>"
    exit 1
fi

file="$1"
current_year=$(date +"%Y")

# Check if file exists
if [ ! -f "$file" ]; then
    echo "Error: File $file not found"
    exit 1
fi

# Update copyright years with company name
# Matches any year or year range between © and the rest of the line
sed -i.bak -E "s/(© )([0-9]{4})([-][0-9]{4})?/\1\2-$current_year/" "$file"

# Remove backup file
rm "${file}.bak"

echo "Copyright years updated in $file"