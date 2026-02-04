# Brew Analysis Learnings

## Conventions

- `brew info --json=v2 --installed` is the most efficient way to get data for all packages at once.
- `brew leaves` is the best source of truth for "what did the user explicitly install".
- The ratio of 1:6 (leaves:dependencies) is typical for a dev environment.

## Patterns

- Categorization by keyword works surprisingly well for standard dev tools.
- "Cleanup Candidates" logic (finding libs in leaves) yielded 0 results, suggesting `brew autoremove` or similar might have been run previously, or the user is just tidy.
