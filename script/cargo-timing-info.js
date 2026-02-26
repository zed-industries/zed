#!/usr/bin/env node

const fs = require("fs");
const os = require("os");
const path = require("path");

function getZedDataDir() {
  const platform = process.platform;

  if (platform === "darwin") {
    // macOS: ~/Library/Application Support/Zed
    return path.join(os.homedir(), "Library", "Application Support", "Zed");
  } else if (platform === "linux" || platform === "freebsd") {
    // Linux/FreeBSD: $FLATPAK_XDG_DATA_HOME or XDG_DATA_HOME/zed
    if (process.env.FLATPAK_XDG_DATA_HOME) {
      return path.join(process.env.FLATPAK_XDG_DATA_HOME, "zed");
    }
    const xdgDataHome = process.env.XDG_DATA_HOME || path.join(os.homedir(), ".local", "share");
    return path.join(xdgDataHome, "zed");
  } else if (platform === "win32") {
    // Windows: LocalAppData/Zed
    const localAppData = process.env.LOCALAPPDATA || path.join(os.homedir(), "AppData", "Local");
    return path.join(localAppData, "Zed");
  } else {
    // Fallback to XDG config dir
    const xdgConfigHome = process.env.XDG_CONFIG_HOME || path.join(os.homedir(), ".config");
    return path.join(xdgConfigHome, "zed");
  }
}

function extractUnitData(htmlContent) {
  // Find the UNIT_DATA array in the file
  const unitDataMatch = htmlContent.match(/const\s+UNIT_DATA\s*=\s*(\[[\s\S]*?\]);/);
  if (!unitDataMatch) {
    throw new Error("Could not find UNIT_DATA in the file");
  }

  try {
    return JSON.parse(unitDataMatch[1]);
  } catch (e) {
    throw new Error(`Failed to parse UNIT_DATA as JSON: ${e.message}`);
  }
}

function formatTime(seconds) {
  if (seconds < 60) {
    return `${seconds.toFixed(2)}s`;
  }
  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = seconds % 60;
  return `${minutes}m ${remainingSeconds.toFixed(2)}s`;
}

function formatUnit(unit) {
  let name = `${unit.name} v${unit.version}`;
  if (unit.target && unit.target.trim()) {
    name += ` (${unit.target.trim()})`;
  }
  return name;
}

function parseTimestampFromFilename(filePath) {
  const basename = path.basename(filePath);
  // Format: cargo-timing-20260219T161555.879263Z.html
  const match = basename.match(/cargo-timing-(\d{4})(\d{2})(\d{2})T(\d{2})(\d{2})(\d{2})\.(\d+)Z\.html/);
  if (!match) {
    return null;
  }
  const [, year, month, day, hour, minute, second, microseconds] = match;
  // Convert to ISO 8601 format
  const milliseconds = Math.floor(parseInt(microseconds) / 1000);
  return `${year}-${month}-${day}T${hour}:${minute}:${second}.${milliseconds.toString().padStart(3, "0")}Z`;
}

function writeBuildTimingJson(filePath, durationMs, firstCrate, target, blockedMs, command) {
  const buildTimingsDir = path.join(getZedDataDir(), "build_timings");

  // Create directory if it doesn't exist
  if (!fs.existsSync(buildTimingsDir)) {
    fs.mkdirSync(buildTimingsDir, { recursive: true });
  }

  // Parse timestamp from filename, or use file modification time as fallback
  let startedAt = parseTimestampFromFilename(filePath);
  if (!startedAt) {
    const stats = fs.statSync(filePath);
    startedAt = stats.mtime.toISOString();
  }

  const buildTiming = {
    started_at: startedAt,
    duration_ms: durationMs,
    first_crate: firstCrate,
    target: target,
    blocked_ms: blockedMs,
    command: command,
  };

  const jsonPath = path.join(buildTimingsDir, `build-timing-${startedAt}.json`);
  fs.writeFileSync(jsonPath, JSON.stringify(buildTiming, null, 2) + "\n");
  console.log(`\nWrote build timing JSON to: ${jsonPath}`);
}

function analyzeTimings(filePath, command) {
  // Read the file
  const htmlContent = fs.readFileSync(filePath, "utf-8");

  // Extract UNIT_DATA
  const unitData = extractUnitData(htmlContent);

  if (unitData.length === 0) {
    console.log("No units found in UNIT_DATA");
    return;
  }

  // Find the unit that finishes last (start + duration)
  let lastFinishingUnit = unitData[0];
  let maxEndTime = unitData[0].start + unitData[0].duration;

  for (const unit of unitData) {
    const endTime = unit.start + unit.duration;
    if (endTime > maxEndTime) {
      maxEndTime = endTime;
      lastFinishingUnit = unit;
    }
  }

  // Find the first crate that had to be rebuilt (earliest start time)
  // Sort by start time to find the first one
  const sortedByStart = [...unitData].sort((a, b) => a.start - b.start);
  const firstRebuilt = sortedByStart[0];

  // The minimum start time indicates time spent blocked (e.g. waiting for cargo lock)
  const blockedTime = firstRebuilt.start;

  // Find the last item being built (the one that was still building when the build finished)
  // This is the unit with the latest end time (which we already found)
  const lastBuilding = lastFinishingUnit;

  console.log("=== Cargo Timing Analysis ===\n");
  console.log(`File: ${path.basename(filePath)}\n`);
  console.log(`Total build time: ${formatTime(maxEndTime)}`);
  console.log(`Time blocked: ${formatTime(blockedTime)}`);
  console.log(`Total crates compiled: ${unitData.length}\n`);
  console.log(`First crate rebuilt: ${formatUnit(firstRebuilt)}`);
  console.log(`  Started at: ${formatTime(firstRebuilt.start)}`);
  console.log(`  Duration: ${formatTime(firstRebuilt.duration)}\n`);
  console.log(`Last item being built: ${formatUnit(lastBuilding)}`);
  console.log(`  Started at: ${formatTime(lastBuilding.start)}`);
  console.log(`  Duration: ${formatTime(lastBuilding.duration)}`);
  console.log(`  Finished at: ${formatTime(lastBuilding.start + lastBuilding.duration)}`);

  // Write JSON file for BuildTiming struct
  const durationMs = maxEndTime * 1000;
  const blockedMs = blockedTime * 1000;
  const firstCrateName = firstRebuilt.name;
  const targetName = lastBuilding.name;
  writeBuildTimingJson(filePath, durationMs, firstCrateName, targetName, blockedMs, command);
}

// Main execution
const args = process.argv.slice(2);

if (args.length === 0) {
  console.error("Usage: cargo-timing-info.js <path-to-cargo-timing.html> [command]");
  console.error("");
  console.error("Example:");
  console.error("  cargo-timing-info.js target/cargo-timings/cargo-timing-20260219T161555.879263Z.html");
  process.exit(1);
}

const filePath = args[0];
const command = args[1] || null;

if (!fs.existsSync(filePath)) {
  console.error(`Error: File not found: ${filePath}`);
  process.exit(1);
}

try {
  analyzeTimings(filePath, command);
} catch (e) {
  console.error(`Error: ${e.message}`);
  process.exit(1);
}
