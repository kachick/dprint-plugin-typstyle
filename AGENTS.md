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
