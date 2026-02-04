# Draft: Brew List Analysis

## Context
- User wants to know what's in their `brew list` and what each package does.
- Preliminary check found ~27 top-level packages (leaves) and ~160 total.

## Scope
- **Selected**: Full Deep Dive (~160 packages).
- **Format**: Categorized Report.
- **Goal**: Explanation + Cleanup Candidates.

## Technical Approach
1. **Data Gathering**: `brew info --json=v2 --installed` (Batch).
2. **Analysis**:
   - Parse JSON.
   - Build dependency graph to confirm "leaf" status.
   - **Categorization**: Use keyword heuristics + agent classification.
   - **Cleanup Detection**: Identify leaves that haven't been used (hard to tell usage) or are likely accidental installs.
3. **Output**: `BREW_REPORT.md`

## Sections
1. **Executive Summary**
2. **Cleanup Recommendations** (Orphaned? Unused?)
3. **Top-Level Tools** (Categorized)
4. **Libraries & Dependencies** (Grouped)

