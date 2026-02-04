# Plan: Brew List Analysis & Cleanup Report

## TL;DR

> **Quick Summary**: Analyze ~160 installed Homebrew packages, categorize them by function (using `brew info` metadata), and generate a readable Markdown report with cleanup recommendations.
>
> **Deliverables**:
> - `BREW_REPORT.md`: Categorized list + Cleanup Candidates.
> - `analyze_brew.py`: Script used for analysis (reusable).
>
> **Estimated Effort**: Quick (1-2 tasks)
> **Parallel Execution**: Sequential
> **Critical Path**: Dump JSON → Parse & Generate Report

---

## Context

### Original Request
User ("ulw") wants to check `brew list`, explain what packages do, and identify cleanup candidates.

### Interview Summary
**Key Decisions**:
- **Scope**: Full Deep Dive (~160 packages).
- **Format**: Categorized Markdown Report (not just a flat list).
- **Cleanup**: Identify unused/orphaned packages (Candidates only - NO auto-delete).

**Research Findings**:
- ~27 top-level "leaves" (user installed).
- ~130 dependencies.
- `brew info --json=v2 --installed` provides all necessary metadata including descriptions and dependencies.

### Metis Review (Simulated)
**Guardrails Applied**:
- **Read-Only**: Agent MUST NOT run `brew uninstall`. Only list commands.
- **Accuracy**: Use `brew info` description as source of truth.
- **Casks**: Include Casks in analysis if present.

---

## Work Objectives

### Core Objective
Generate a clear, categorized report of all installed software to help the user understand their system.

### Concrete Deliverables
- `BREW_REPORT.md` (The final report)
- `brew_data.json` (Raw data, for reference)

### Definition of Done
- [x] `BREW_REPORT.md` exists and is non-empty.
- [x] Report contains "Executive Summary", "Cleanup Candidates", and Categorized lists.
- [x] No packages were deleted during the process.

### Must Have
- Descriptions for every package.
- Clear distinction between "Leaves" (User Installed) and "Dependencies".

### Must NOT Have (Guardrails)
- **NO** deletion of files or packages.
- **NO** external scraping (use `brew info` data only).

---

## Verification Strategy

> **UNIVERSAL RULE: ZERO HUMAN INTERVENTION**
> ALL tasks must be verifiable by the agent.

### Test Decision
- **Infrastructure exists**: N/A (Script-based task).
- **Automated tests**: NO (One-off analysis).
- **Agent-Executed QA**: YES (Mandatory).

### Agent-Executed QA Scenarios

**Scenario: Report Generation Verification**
- **Tool**: Bash
- **Steps**:
    1. Run analysis script.
    2. Check `BREW_REPORT.md` exists.
    3. Grep for "Executive Summary".
    4. Grep for "Cleanup Candidates".
    5. Count lines (should be > 160).
- **Evidence**: `head -n 20 BREW_REPORT.md`

---

## TODOs

- [x] 1. Generate Brew Analysis Report

  **What to do**:
  1. Run `brew info --json=v2 --installed > brew_data.json` to get raw data.
  2. Create a Python script `analyze_brew.py` that:
     - Loads `brew_data.json`.
     - Identifies **Leaves** (packages not in the `dependencies` list of any other installed package).
     - Identifies **True Orphans** (Leaves that are marked as dependencies but have no parents? Or just Leaves that are NOT in `brew leaves`? actually `brew leaves` gives the source of truth for "requested" packages. We should use `brew leaves` output to cross-reference).
     - **Categorization Logic**:
       - Create simple keyword buckets (e.g., "Languages": [go, python, node], "Media": [ffmpeg, imagemagick], "Libs": [lib*, openssl]).
       - Default to "Uncategorized/Tools".
     - Generates `BREW_REPORT.md` with:
       - **Summary**: Total count, Leaf count, Size (if available).
       - **Cleanup Candidates**: Leaves that are NOT in the `brew leaves` list (if any - unlikely if `brew leaves` is accurate) OR Leaves that look like libs but are leaves.
       - **Detailed List**: Grouped by Category. Format: `**name** (v1.2): Description [Homepage]`.
  3. Run the script.
  4. Verify output.

  **Recommended Agent Profile**:
  - **Category**: `quick` (Scripting task)
  - **Skills**: [`python`] (for robust JSON parsing)

  **References**:
  - `brew info --json=v2 --installed` output structure:
    ```json
    {
      "formulae": [
        { "name": "ack", "desc": "Search tool...", "homepage": "...", "dependencies": [], "installed": [{"size": 123}] }
      ],
      "casks": []
    }
    ```

  **Acceptance Criteria**:
  - [x] `brew_data.json` exists
  - [x] `analyze_brew.py` exists
  - [x] `BREW_REPORT.md` generated
  - [x] Report contains sections: "Executive Summary", "Leaves (User Installed)", "Dependencies"

  **Agent-Executed QA Scenarios**:
  ```
  Scenario: Verify Report Content
    Tool: Bash
    Preconditions: analyze_brew.py ran successfully
    Steps:
      1. test -f BREW_REPORT.md
      2. grep -q "# Brew Analysis Report" BREW_REPORT.md
      3. grep -q "## Leaves" BREW_REPORT.md
      4. grep -q "## Dependencies" BREW_REPORT.md
      5. head -n 5 BREW_REPORT.md
    Expected Result: Report exists with correct headers
    Evidence: Output of head command
  ```

  **Commit**: YES
  - Message: `docs: generate brew analysis report`
  - Files: `analyze_brew.py`, `BREW_REPORT.md`

---

## Success Criteria

### Verification Commands
```bash
ls -lh BREW_REPORT.md
head -n 10 BREW_REPORT.md
```

### Final Checklist
- [x] Report covers all ~160 packages
- [x] Cleanup candidates identified
- [x] No system changes made
