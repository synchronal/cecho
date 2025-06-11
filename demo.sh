#!/bin/bash

# Demo script for cecho - a colorized text printing tool
# This script demonstrates various features and capabilities of cecho

set -e

echo "=== CECHO DEMO ==="
echo "This demo showcases the colorized text printing capabilities of cecho"
echo ""

# Build the project if needed
if [ ! -f "target/release/reko" ]; then
    echo "Building cecho..."
    mise exec -- cargo build --release
    echo ""
fi

CECHO="./target/release/reko"

echo "--- Quick Demo ---"
echo "Here's a quick taste of what cecho can do:"
$CECHO "Hello {red: world}! This is {blue bold: amazing} text with {green underline: colors}!"
$CECHO "Status: {green: ✓ Success} | Warning: {yellow: ⚠ Check this} | Error: {red bold: ✗ Failed}"
echo "Now let's see all the features in detail..."
echo ""

echo "--- Basic Colors ---"
$CECHO "This is {red: red text}"
$CECHO "This is {green: green text}"
$CECHO "This is {blue: blue text}"
$CECHO "This is {yellow: yellow text}"
$CECHO "This is {magenta: magenta text}"
$CECHO "This is {cyan: cyan text}"
$CECHO "This is {white: white text}"
$CECHO "This is {black: black text}"
echo ""

echo "--- Text Styles ---"
$CECHO "This is {bold: bold text}"
$CECHO "This is {italic: italic text}"
$CECHO "This is {underline: underlined text}"
$CECHO "This is {dim: dimmed text}"
$CECHO "This is {blink: blinking text}"
$CECHO "This is {reverse: reversed text}"
$CECHO "This is {strikethrough: strikethrough text}"
echo ""

echo "--- Combining Colors and Styles ---"
$CECHO "This is {red bold: red and bold}"
$CECHO "This is {blue underline: blue and underlined}"
$CECHO "This is {green italic: green and italic}"
$CECHO "This is {yellow bold underline: yellow, bold, and underlined}"
$CECHO "This is {magenta bold italic underline: magenta with multiple styles}"
echo ""

echo "--- Multiple Formatted Sections ---"
$CECHO "Hello {red: world}, this is {blue: amazing} and {green bold: fantastic}!"
$CECHO "Status: {green bold: SUCCESS} - Operation completed {cyan: successfully}"
$CECHO "Warning: {yellow: Please check} your {red underline: configuration} file"
echo ""

echo "--- Practical Examples ---"
$CECHO "🚀 {green bold: Build Status:} {green: PASSED}"
$CECHO "📊 {blue bold: Test Results:} {cyan: 45 passed}, {yellow: 2 skipped}, {red: 0 failed}"
$CECHO "⚠️  {yellow bold: Warning:} {yellow: Deprecated function detected in} {red underline: module.rs:42}"
$CECHO "✅ {green bold: Deployment:} {green: Successfully deployed to} {cyan: production}"
$CECHO "🔧 {blue bold: Debug:} {dim: Processing file} {white: /path/to/file.txt}"
echo ""

echo "--- Log-like Output ---"
$CECHO "[{green: INFO}] Application started successfully"
$CECHO "[{yellow: WARN}] {yellow: Connection timeout, retrying...}"
$CECHO "[{red: ERROR}] {red bold: Failed to connect to database}"
$CECHO "[{blue: DEBUG}] {dim: Variable x =} {white: 42}"
echo ""

echo "--- Using -n flag (no newline) ---"
echo "Printing without newlines:"
$CECHO -n "Loading"
for i in {1..5}; do
    sleep 0.3
    $CECHO -n "{cyan: .}"
done
$CECHO " {green bold: Done!}"
echo ""

echo "--- Complex Formatting ---"
$CECHO "The {cyan: hovercraft} is {red underline: full of eels}!"
$CECHO "My {blue bold italic: favorite} programming language is {green: Rust} 🦀"
$CECHO "Error code: {red bold: 404} - {yellow: File not found} at {dim: /var/log/app.log}"
echo ""

echo "--- Mixed Plain and Formatted Text ---"
$CECHO "Regular text with {red: colored} words and more regular text"
$CECHO "Start {green bold: middle} end"
$CECHO "No formatting here at all"
echo ""

echo "--- Edge Cases ---"
$CECHO "Curly braces without colon: {just text}"
$CECHO "Empty formatting: {: just text}"
$CECHO "Unknown format: {unknown: this text} should appear normally"
echo ""

echo "=== DEMO COMPLETE ==="
echo "cecho supports:"
echo "• Colors: red, green, blue, yellow, magenta, cyan, white, black"
echo "• Styles: bold, italic, underline, dim, blink, reverse, strikethrough"
echo "• Multiple formats can be combined with spaces"
echo "• Multiple formatted sections in one string"
echo "• -n flag to suppress newline"
echo ""
echo "Usage: cecho [-n] '<text with {format: content} sections>'"