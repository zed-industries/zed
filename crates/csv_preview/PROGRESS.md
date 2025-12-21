# CSV Preview Progress Tracker

This document outlines the current state of the POC feature. It might not be the most understandable document for others, but the feature author depends on it to track progress and ideas.

## Problems to Discuss
- **Data table architecture**: Replace const generics with flexible types (array/vec) to support both compile time fix column sizes and runtime defined column numbers

## Progress

### Table: UI/Behavior

**Features:**
- [ ] feat: Add multiline cells (variable list) / single line cells (unified list) modes
- [ ] feat: Add tooltip for truncated values / show full value on selection
- [ ] feat: Optional headers (c1 c2 c3) toggle
- [x] feat: Add performance metrics (timings for ordering, parsing, copying, selection, etc)
- [ ] feat: Horizontal scroll vs fit all columns
- [ ] feat: Update column resizing behavior (double-click column boundary to fit to content, again to fit to column name, again to reset)
- [ ] feat: Add persisting of the state of opened previews on editor restart
- [ ] feat: Add `CsvPreviewView` settings persistence (probably need to add them to settings.json)
- [ ] feat: Generalize the preview to support .tsv files (maybe even all files, and prompt user to select format if it's not .csv/.tsv)
- [ ] feat: Smart debouncing (now + cooldown instead of dumb waiting)

**Fixes:**
- [x] fix: Vertical scrolling doesn't work in variable list rendering mode

### Table: Minor UI Tweaks

**Features:**
- [ ] feat: Make uniform background + add horizontal lines between cells
- [x] feat: Monospace font (with toggle)

**Fixes:**

### Row Identifiers (Line Numbers)

**Features:**
- [x] feat: Make width not resizable manually, but adjust to file size (number of lines)
- [x] feat: Highlight number of selected (focused) line
- [x] feat: Add numbering type: lines/rows

**Fixes:**
- [ ] fix: Calculate width of a character (currently hardcoded)
- [x] fix: Update implementation to correspond to actual numbers in the source code (broken in multiline CSV rows)

### Selection

**Features:**
- [x] feat: Update paddings (same for selected and not selected columns)
- [x] feat: Add multiple selections using cmd modifier
- [x] feat: Clear selection
- [x] feat: Navigate with keyboard
- [x] feat: Extend selection with keyboard
- [x] feat: Optimize selection by using `SelectionStrategy`:
  ```rust
  enum SelectionStrategy {
      /// Whole document (CMD+A)
      AllCells,
      /// Single cell or single range selected
      SingleChunk(Chunk),
      /// Multiple chunks selected while holding CMD
      MultipleChunks(Vec<Chunk>)
  }
  enum Chunk {
      /// Single cell
      Cell(DisplayCellId),
      /// Square range of cells
      Range(DisplayCellId, DisplayCellId)
  }
  ```
  - [ ] feat: Convert adjacent cells selected individually into a range if range is single dimention (1xN or Nx1)
- [ ] feat: Move viewport by keyboard selection
- [ ] feat: Move viewport by mouse selection

**Fixes:**
- [x] fix: Selection when sorted (follow data cells not display cells)

### Copy Selected

**Features:**
- [x] feat: Add copying by Ctrl+C
- [x] feat: Add copy as (CSV, TSV, markdown table, semicolon separated)
- [ ] feat: Copy in display mode (what you see) not data mode (how it's in file), or toggle between them

**Fixes:**

### Ordering

**Features:**
- [ ] feat: Add tooltip on order button
- [ ] feat: Add manual ordering of columns

**Fixes:**

### Filtering

**Features:**
- [ ] feat: Implement initial filtering by column
- [ ] feat: Add search bar for filtering modal

**Fixes:**

### Editing

**Features:**
- [ ] feat: Inline edits (click cell to enter editor mode. Edit text, hit enter, and changes applied to original file)
- [ ] feat: Apply ordering settings (permanently write to buffer current rows and columns order)
- [ ] feat: Paste from clipboard over several cells (parse based on defined separator). Allow only in `AllCells` / `SingleChunk` selection modes

**Fixes:**
