// Weekly review data — one entry per week, newest last.
// The Week Review skill appends a new object to this array each week.
//
// Schema (per week):
// {
//   id: "2026_29_Jul",       // week id = weekly filename stem
//   week: 29,                // the WW number
//   label: "Week 29",
//   range: "Jul 13 – Jul 19, 2026",
//   status: "reviewed",      // "reviewed", or "in-progress" if the week isn't over
//   goals:     [ { text: "…", done: true } ],   // from # Week Goals
//   tentative: [ { text: "…", done: false } ],  // from # Tentative (or [])
//   personal:  [ { text: "…", done: false } ],  // from # Personal (or [])
//   highlights: [ "…" ],     // 2–3 entries, or [] for an in-progress week
//   projects: [
//     { name: "Scheduler", goal: true, tasks: [ "task one" ] }  // omit `goal` when not a week goal
//   ],
//   prs: {
//     created:  [ { ref: "OpenCue#2425", title: "…", status: "open", src: "github" } ],
//     reviewed: [ { ref: "spi-centos!18", title: "…", src: "gitlab" } ]
//   }
// }
window.WEEKS = [];
