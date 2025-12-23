# CSV Preview Progress Tracker

This document outlines the current state of the POC feature. It might not be the most understandable document for others, but the feature author depends on it to track progress and ideas.

## Problems to Discuss
**Data table limitations**:
- Const generics. Switch to dynamic arrays
- Awkward table width configuration (can't really make the table wider than parent automatically). If width is set statically to be bigger than the parent, scrolling needs to be enabled separately, despite it's stated, that it should work automatically.
- Column widths are calculated via %, and not absolute units. This way it's not possible to calculate whole table based on width of each column.
- Can't pin first column (row numbers). This results in bad UX when scrolling horizontally (row numbers move out of viewport, and it becomes hard to reason about rows)

**Column Resizing Behavior: Current vs. Needed**

- **Current approach:**
  The data table uses **proportional (fluid) resizing**. When you adjust a column’s width, the space is redistributed among other columns so the overall table width stays the same.

- **Limitations:**
  - The table’s width cannot be determined by simply summing up individual column widths.
  - The table cannot naturally expand beyond its parent container.
  - This makes it difficult to handle CSV files with columns of widely varying content lengths.

- **Desired approach:**
  **Independent (absolute) resizing**—like in Google Sheets or Excel:
  - Each column can be resized without affecting others.
  - The table’s total width grows or shrinks as columns are resized.
  - This allows the table to expand beyond its parent and enables intuitive horizontal scrolling.

## Progress

### Table: UI/Behavior

**Features:**
- [x] feat: Smart debouncing (now + cooldown instead of dumb waiting)
- [x] feat: Add tooltip to show full value on hover
- [x] feat: Add performance metrics (timings for ordering, parsing, copying, selection, etc)
- [ ] feat: Add multiline cells (variable list) / single line cells (unified list) modes
- [ ] feat: Optional headers (c1 c2 c3) toggle
- [ ] feat: Horizontal scroll vs fit all columns (**need infra**)
- [ ] feat: Update column resizing behavior (double-click column boundary to fit to content, again to fit to column name, again to reset)
- [ ] feat: Add persisting of the state of opened previews on editor restart
- [ ] feat: Add `CsvPreviewView` settings persistence (probably need to add them to settings.json)
- [ ] feat: Generalize the preview to support .tsv files (maybe even all files, and prompt user to select format if it's not .csv/.tsv)

**Fixes:**
- [x] fix: Vertical scrolling doesn't work in variable list rendering mode
  - [ ] fix: Variable and Uniform lists have the separate scrolls. TODO: sync them

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
- [x] feat: Move viewport by keyboard selection
- [x] feat: Move viewport by mouse selection

**Fixes:**
- [x] fix: Selection when sorted (follow data cells not display cells)

### Copy Selected

**Features:**
- [x] feat: Add copying by Ctrl+C
- [x] feat: Add copy as (CSV, TSV, markdown table, semicolon separated)
- [x] feat: Copy in display mode (what you see) not data mode (how it's in file), or toggle between them

**Fixes:**

### Ordering

**Features:**
- [x] feat: Add tooltip on order button
- [ ] feat: Add manual ordering of columns by dragging (**needs infra**)

**Fixes:**

### Filtering

**Features:**
- [ ] feat: Implement initial filtering by column
- [ ] feat: Add search bar for filtering popover

**Fixes:**

### Editing

**Features:**
- [x] feat: Inline edits (click cell to enter editor mode. Edit text, hit enter, and changes applied to original file)
  - [x] fix: Allow overflow for the editor. Style editor to have background, border and paddings
    - [ ] fix: Add z-index, to be on top of adjacent cells
- [ ] feat: Apply ordering settings (permanently write to buffer current rows and columns order)
- [ ] feat: Paste from clipboard over several cells (parse based on defined separator). Allow only in `AllCells` / `SingleChunk` selection modes

**Fixes:**
