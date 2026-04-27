# AGENTS.md

## Purpose

This document defines strict operational rules for AI agents contributing to the Nevo repository.
Agents must follow these rules to ensure safe, minimal, and correct code changes.

---

## Core Principles

### 1. Issue-Driven Work

* Every task is tied to a GitHub issue
* The issue defines:

  * Scope
  * Target folder
  * Expected outcome
* Do not infer or expand beyond the issue

---

## Folder Identification (MANDATORY)

Before making any changes, the agent must:

1. Identify which folder the issue belongs to:

   * `Frontend`
   * `Backend`
   * `Contract`

2. Restrict all actions to that folder only

> ❗ NEVER modify multiple folders in a single task

---

## Allowed Modifications

### Frontend (Next.js)

* Modify only:

  * The relevant page folder, OR
  * The corresponding `page.tsx`
* Ensure:

  * Build succeeds
  * UI remains responsive
  * Code formatting is consistent

---

### Backend (NestJS)

* Modify only:

  * Specific module, controller, or service tied to the issue
* Ensure:

  * Build succeeds
  * Formatting is correct

---

### Contract

* Modify only:

  * Main contract logic file/module
  * Relevant test files
* Ensure:

  * Contracts compile successfully
  * All tests pass
  * Formatting is correct

> ⚠️ Contract changes require strict minimalism and correctness

---

## Strict Constraints

### Minimal Scope Enforcement

* Do NOT:

  * Refactor unrelated code
  * Rename unrelated files
  * Introduce unrelated improvements
* Only implement what is explicitly required

### No Cross-Layer Changes

* NEVER combine:

  * Frontend + Backend
  * Backend + Contract
  * Frontend + Contract

---

## Handling Dependencies & Unimplemented Features

### Scenario 1: Feature Not Implemented

If the current issue requires a feature (from another GitHub issue) that hasn't been implemented yet:

* **Do NOT wait** for the other issue to be completed
* Instead, create a **minimal mock** of only what is needed
* The mock should:
  * Provide exactly the API or interface required
  * Return hardcoded or placeholder data
  * Be clearly marked as a mock (in comments, type names like `MockService`, or function names)
  * NOT attempt to implement the full feature from the dependent issue
* Concrete examples:
  * **Backend API dependency**: Create an endpoint that returns hardcoded JSON data
  * **Contract function dependency**: Create a function that returns sample contract data
  * **External service dependency**: Mock the API responses with hardcoded data
  * **Database queries**: Mock the query results with sample data

### Scenario 2: Issue Explicitly Depends on Another Issue

When an issue requires functionality explicitly linked in another GitHub issue:

* Create a **minimal mock version** of that functionality
* The mock only needs to provide the minimal data or behavior the current issue requires
* Clearly mark it: `// TODO: Replace with real implementation from issue #XYZ`
* Do NOT implement the full scope of the dependent issue
* Once the dependent issue is completed, the real implementation will replace the mock

### Key Points

* Agents should **complete issues independently** without waiting for dependencies
* Mocks are **temporary and clearly marked**
* Focus on delivering the feature's functionality using mock data
* Maintainers will handle replacing mocks with real implementations later

---

## Build & Validation Requirements

Before completing a task, the agent must ensure:

* Relevant project builds successfully
* Tests pass (for contract work)
* No syntax or formatting errors exist

---

## Code Style

* Follow existing patterns in the repository
* Match formatting, naming, and structure
* Do not introduce new patterns unless required

---

## Safety Rules

* Do not modify sensitive logic unless explicitly required
* Avoid introducing breaking changes
* Preserve backward compatibility unless instructed otherwise

---

## Output Expectations

When completing a task, the agent should:

* Produce minimal diffs
* Clearly reflect issue requirements
* Avoid unnecessary verbosity in code changes

---

## Summary

AI agents must:

1. Identify the correct folder from the issue
2. Modify only necessary files within that folder
3. Ensure build/tests pass
4. Follow formatting and existing style
5. Avoid unrelated or cross-folder changes

Failure to follow these rules will result in rejection of the contribution.

---
