# Daily & Weekly (the Timeline Area)

This Area closes the loop on your daily and weekly rhythm: it turns a week of
notes and code activity into a reviewed, visualized record.

The Timeline navigator itself — Today, Yesterday, This Week, Last Week — is a
core part of BreadPaper and always on. This Area layers two things on top of
it: the **Week Review** skill and the **weekly progress dashboard**.

## The Week Review skill

`skills/timeline/week-review.md`

A ritual your LLM runs once a week. It:

- **Reads** the week's daily and weekly notes (`daily/**`, `weekly/**`) and
  your pull and merge requests on GitHub and GitLab.
- **Writes** by appending only: a `# AI Week Review` section at the end of the
  weekly note, and one week object to the dashboard feed
  (`_weekly/site/data.js`).

It never overwrites or deletes anything you wrote — augmentation, not
replacement. Open the skill file to read (or edit) exactly what it does.

## The weekly dashboard

`_weekly/site/index.html`

A static page that computes its own analytics from the feed in `data.js`:
per-week stats and sparklines, goal completion, work grouped by project, and
warnings —

- **time sink** — a large share of the week's work went to projects outside
  your goals;
- **carry-over** — a goal has stayed unfinished across consecutive weeks;
- **lingering** — a project drags on for weeks with minimal progress and was
  never elevated to a week goal.

Click **Weekly Dashboard** in the Areas section to open it in your browser.
It starts empty; each Week Review appends one entry to the feed.

## Where things live

- `skills/timeline/week-review.md` — the Week Review skill (a plain, editable
  markdown file).
- `_weekly/site/` — the dashboard page and its data feed.
- `.breadpaper/areas/timeline/manifest.toml` — the installed record of what
  this Area shipped.

Removing this Area never touches your notes, and any shipped file you have
edited is kept.
