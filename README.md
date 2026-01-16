# Acty

**Acty** is a simple, lightweight, and efficient CLI action logger for developers. It helps you track your daily activities, thoughts, and tasks directly from your terminal, without breaking your flow.

## Features

*   **Fast Logging**: Log activities with a single command.
*   **Tagging**: Categorize logs with tags.
*   **Contextual Timeline**: View time gaps between logs to understand time usage per context (tag).
*   **Search**: Full-text search across content and tags.
*   **Edit & Delete**: Modify or remove logs easily (supports multiple deletion).
*   **Copy**: Duplicate past logs to reuse content.
*   **Archive**: Move old logs to a separate file to keep the active log lightweight.
*   **Markdown Export**: Output logs in Markdown table format for reports.

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/acty.git
cd acty

# Build and install (requires Rust/Cargo)
make install
```

The binary will be installed to `~/.local/bin/acty`. Ensure this directory is in your `PATH`.

## Usage

### 1. Log an Action

```bash
acty log "Started working on feature X" -t work,dev
acty log "Coffee break" -t break
```

### 2. List Logs

View your logs with IDs, timestamps, and time gaps.

```bash
acty list
```

**Filter by Tag (Context Mode):**
When filtering by a tag, `acty` calculates the time gap between the displayed logs, effectively showing the duration spent within that context.

```bash
acty list -t work
```

**Filter by Date/Search:**
```bash
acty list --date 2023-10-27
acty list --search "feature"
```

### 3. Edit Logs

Use the ID from `list` to edit a log. You can use `last` to refer to the most recent log.

```bash
# Edit content only
acty edit 1 "Updated content"

# Edit content and tags (overwrites tags)
acty edit last "Fixed typo" -t work,fix
```

### 4. Delete Logs

```bash
# Delete single log
acty delete 1

# Delete multiple logs
acty delete 1 3 5

# Delete the last log
acty delete last
```

### 5. Copy Logs

Duplicate an existing log entry as a new entry with the current timestamp.

```bash
# Copy content and tags from ID 1
acty copy 1

# Copy tags from ID 1 but change content
acty copy 1 "New task with same context"

# Copy the last log
acty copy last
```

### 6. Manage Tags

See all tags and their usage counts.

```bash
acty tags
```

### 7. Archive Old Logs

Move logs older than 7 days (default) to `archive.json`.

```bash
# Archive logs older than 7 days
acty archive

# Archive logs older than 30 days
acty archive 30
```

To view archived logs, add the `-a` or `--archive` flag to `list`, `search`, `tags`, or `mdt` commands.

```bash
acty list -a
acty search "old bug" -a
```

### 8. Export to Markdown

Generate a Markdown table for your daily report.

```bash
acty mdt --date today > report.md
```

## Configuration

The log file is stored at `~/.local/share/acty/action_log.json` (Linux) by default.
You can customize this by creating `~/.config/acty/config.toml`.

## License

MIT