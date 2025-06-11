# cecho

A command-line tool that makes it easier to print colorized text:

```bash
cecho 'My {cyan: hovercraft} is {red underline: full of eels}!'
```

## Demo Output

=== CECHO DEMO ===  
This demo showcases the colorized text printing capabilities of cecho

--- Quick Demo ---  
Here's a quick taste of what cecho can do:  
Hello <span style="color: red">world</span>! This is <span style="color: blue; font-weight: bold">amazing</span> text with <span style="color: green; text-decoration: underline">colors</span>!  
Status: <span style="color: green">✓ Success</span> | Warning: <span style="color: #b8860b">⚠ Check this</span> | Error: <span style="color: red; font-weight: bold">✗ Failed</span>  
Now let's see all the features in detail...

--- Basic Colors ---  
This is <span style="color: red">red text</span>  
This is <span style="color: green">green text</span>  
This is <span style="color: blue">blue text</span>  
This is <span style="color: #b8860b">yellow text</span>  
This is <span style="color: magenta">magenta text</span>  
This is <span style="color: cyan">cyan text</span>  
This is <span style="color: white">white text</span>  
This is <span style="color: black">black text</span>

--- Text Styles ---  
This is <span style="font-weight: bold">bold text</span>  
This is <span style="font-style: italic">italic text</span>  
This is <span style="text-decoration: underline">underlined text</span>  
This is <span style="opacity: 0.5">dimmed text</span>  
This is <span style="animation: blink 1s linear infinite">blinking text</span>  
This is <span style="filter: invert(1)">reversed text</span>  
This is <span style="text-decoration: line-through">strikethrough text</span>

--- Combining Colors and Styles ---  
This is <span style="color: red; font-weight: bold">red and bold</span>  
This is <span style="color: blue; text-decoration: underline">blue and underlined</span>  
This is <span style="color: green; font-style: italic">green and italic</span>  
This is <span style="color: #b8860b; font-weight: bold; text-decoration: underline">yellow, bold, and underlined</span>  
This is <span style="color: magenta; font-weight: bold; font-style: italic; text-decoration: underline">magenta with multiple styles</span>

--- Multiple Formatted Sections ---  
Hello <span style="color: red">world</span>, this is <span style="color: blue">amazing</span> and <span style="color: green; font-weight: bold">fantastic</span>!  
Status: <span style="color: green; font-weight: bold">SUCCESS</span> - Operation completed <span style="color: cyan">successfully</span>  
Warning: <span style="color: #b8860b">Please check</span> your <span style="color: red; text-decoration: underline">configuration</span> file

--- Practical Examples ---  
🚀 <span style="color: green; font-weight: bold">Build Status:</span> <span style="color: green">PASSED</span>  
📊 <span style="color: blue; font-weight: bold">Test Results:</span> <span style="color: cyan">45 passed</span>, <span style="color: #b8860b">2 skipped</span>, <span style="color: red">0 failed</span>  
⚠️  <span style="color: #b8860b; font-weight: bold">Warning:</span> <span style="color: #b8860b">Deprecated function detected in</span> <span style="color: red; text-decoration: underline">module.rs:42</span>  
✅ <span style="color: green; font-weight: bold">Deployment:</span> <span style="color: green">Successfully deployed to</span> <span style="color: cyan">production</span>  
🔧 <span style="color: blue; font-weight: bold">Debug:</span> <span style="opacity: 0.5">Processing file</span> <span style="color: white">/path/to/file.txt</span>

--- Log-like Output ---  
[<span style="color: green">INFO</span>] Application started successfully  
[<span style="color: #b8860b">WARN</span>] <span style="color: #b8860b">Connection timeout, retrying...</span>  
[<span style="color: red">ERROR</span>] <span style="color: red; font-weight: bold">Failed to connect to database</span>  
[<span style="color: blue">DEBUG</span>] <span style="opacity: 0.5">Variable x =</span> <span style="color: white">42</span>

--- Using -n flag (no newline) ---  
Printing without newlines:  
Loading<span style="color: cyan">.</span><span style="color: cyan">.</span><span style="color: cyan">.</span><span style="color: cyan">.</span><span style="color: cyan">.</span> <span style="color: green; font-weight: bold">Done!</span>

--- Complex Formatting ---  
The <span style="color: cyan">hovercraft</span> is <span style="color: red; text-decoration: underline">full of eels</span>!  
My <span style="color: blue; font-weight: bold; font-style: italic">favorite</span> programming language is <span style="color: green">Rust</span> 🦀  
Error code: <span style="color: red; font-weight: bold">404</span> - <span style="color: #b8860b">File not found</span> at <span style="opacity: 0.5">/var/log/app.log</span>

--- Mixed Plain and Formatted Text ---  
Regular text with <span style="color: red">colored</span> words and more regular text  
Start <span style="color: green; font-weight: bold">middle</span> end  
No formatting here at all

--- Edge Cases ---  
Curly braces without colon: {just text}  
Empty formatting: just text  
Unknown format: this text should appear normally

=== DEMO COMPLETE ===  
cecho supports:  
• Colors: red, green, blue, yellow, magenta, cyan, white, black  
• Styles: bold, italic, underline, dim, blink, reverse, strikethrough  
• Multiple formats can be combined with spaces  
• Multiple formatted sections in one string  
• -n flag to suppress newline

Usage: cecho [-n] '<text with {format: content} sections>'

## Installation

TBD

## Options

TBD