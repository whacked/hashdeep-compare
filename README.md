# hashdeep-compare

utility to compare 2 hashdeep checksum files that are generated against 2 presumably matching directory sources, and finds the differences.

a use case is if you ran hashdeep against a directory at different times, and possibly expect some level of corruption, you can use this to detect changes.

this tool was mostly written by Github Copilot.

use at your own risk. shall you experience any adverse consequences, direct any grievances and consultation requests to ChatGPT

# usage

enter the development environment with `nix develop` if you have flakes enabled, or `nix shell`. Then look for the `compare` alias for usage syntax. It takes a `base_file` and `comp_file`, which should be outputs of a `hashdeep` command.
