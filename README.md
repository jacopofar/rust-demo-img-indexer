# Rust demo: Image indexer/ duplicate finder

## Archived: this repo is archived, it's just a small demo very limited in scope

This is a Rust tool that, given the path of a folder, index every PNG and JPG
image in it into a SQLite database.

The scan is not rcursive, and only files with extension png and jpg are considered.

For each image it stores the path, the width and height and the colors sampled in
a 10x10 grid.

Finding images that are very similar or identical is then trivial with a SQL
query.