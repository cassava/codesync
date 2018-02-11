# Design

## Single Folder

- Is it a Git repository?
- Are there any uncommited changes?
- Are there any untracked files?
- Does the repository have a remote?
- Is that remote one of several *want* hosts, e.g., github or gitlab?
- Is the git repository synchronized?

## Multiple Folders

- Same output as single folders.

## Recursive Analysis

- Find directories that have Git repoitories.
- Flag any siblings that are not repositories.

## Store a Configuration

- Let us remove false positives/negatives via configuration.
- Let us save a set of directories that should be scanned.
- Let us use "profiles".
