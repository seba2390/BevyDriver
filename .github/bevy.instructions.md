# Bevy Documentation Protocol

## Purpose
This document defines the strict procedure for retrieving up-to-date Bevy API information. Follow this protocol whenever you need to verify syntax, structs, or system signatures to ensure the code works with `bevy/latest`.

## Retrieval Steps

### 1. Construct Search Query
When you need to look up a keyword (e.g., `Query`, `Commands`, `add_systems`), construct the search URL by appending the keyword to:
`https://docs.rs/bevy/latest/bevy/?search=<KEYWORD>`

### 2. Navigate and Select
1. Use the `web` tool to access the generated URL.
2. The page will display search results. **Do not stop here.**
3. Identify the most relevant result (prioritize matches in `bevy::prelude` or core modules like `bevy_ecs`) and navigate to that specific API page.

### 3. Extract Technical Data
Once on the specific API page, you must extract:
- **The Definition**: The exact struct/function signature (usually found in the top code block).
- **The Example**: Any provided usage examples.

## Constraints
- **Source of Truth**: You are restricted to `docs.rs/bevy/latest`.
- **No Hallucination**: If the page does not load or the search returns no results, state "Documentation not found." Do not guess the syntax based on older training data.
