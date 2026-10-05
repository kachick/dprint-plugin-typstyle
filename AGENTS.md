# Rules for AI Agents

## Commit Message Trailer

Based on Linux kernel AI attribution guidelines: <https://github.com/torvalds/linux/commit/78d979db6cef557c171d6059cbce06c3db89c7ee>, but simplified by omitting the `TOOLS` list to minimize noise.

ALWAYS append `Assisted-by: AGENT_NAME:MODEL_VERSION` as the last line of the commit message.

- `AGENT_NAME` is the name of the AI tool or framework\
  Dynamically replace `<AGENT_NAME>` with your executed Tool name (e.g., Antigravity)
- `MODEL_VERSION` is the specific model version used\
  Dynamically replace `<MODEL_VERSION>` with the lowercase kebab-case slug of the model you are currently running on (e.g., `gemini-3.8-flash`, `gemini-3-pro`).

Examples

- Assisted-by: Antigravity:gemini-3.8-flash
- Assisted-by: Claude:claude-3-opus

## Testing Policy

Tests bridge specifications and implementation. Tests must not simply copy or repeat the implementation.

- **Complement Each Other Without Duplication**:
  In `git show <rev>`, the commit message, code changes, tests, and comments must all support each other without repeating the same information.
  Tests that only list the same parameters as the code add no value. Having no such test is better for future readability.
- **E2E First, Then Minimal Unit Tests**:
  This does not mean you should avoid writing tests.
  First, test the main happy paths with lightweight E2E tests like external health checks (keep them light and do not make them heavy).
  Next, add only minimal unit tests as needed while writing code to prevent regressions.
- **Never Assume "More Tests Are Better"**:
  NEVER think that having more tests is always better. Absolutely avoid redundant tests.
