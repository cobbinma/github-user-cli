# GitHub User CLI

Given a GitHub username the program will find 10 of that users repositories ranked by stars

For each repository displayed, the following fields are shown:
* Repository name
* Repository URL
* Repository description
* Star count

Results are cached; clear the cache with `-cc`

Output can be saved in both JSON `-j` and YAML `-y` formats. 
Output will be pretty printed to terminal if no formats are specified

A GitHub token can be given to increase available request rate with `-t`

Run program with `cargo run -- -u cobbinma`

Use `cargo run -- -help` to see more information