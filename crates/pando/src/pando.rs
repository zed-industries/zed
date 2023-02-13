//! ## Goals
//! - Opinionated Subset of Obsidian. Only the things that cant be done other ways in zed
//! - Checked in .zp file is an sqlite db containing graph metadata
//! - All nodes are file urls
//! - Markdown links auto add soft linked nodes to the db
//! - Links create positioning data regardless of if theres a file
//! - Lock links to make structure that doesn't rotate or spread
//! - Drag from file finder to pando item to add it in
//! - For linked files, zoom out to see closest linking pando file

//! ## Plan
//! - [ ] Make item backed by .zp sqlite file with camera position by user account
//! - [ ] Render grid of dots and allow scrolling around the grid
//! - [ ] Add scale property to layer canvas and manipulate it with pinch zooming
//! - [ ] Allow dropping files onto .zp pane. Their relative path is recorded into the file along with
