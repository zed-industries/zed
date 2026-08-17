import {
  abort,
  desc,
  env,
  run,
  sh,
  shCapture,
  task,
} from "https://deno.land/x/drake@v1.5.0/mod.ts";

const SHOULD_CARGO_PUBLISH = true;
const SHOULD_PUSH_DOCS_TO_GITHUB_PAGES = false;

desc("Release a new version of the crate");
task("release", [], async () => {
  const version = env("version");
  if (version == null) {
    abort("The version to release was not specified");
  }
  if (!isValidSemVer(version)) {
    abort("The given version is not a valid SemVer string");
  }

  await sh("cargo test --all-features");
  await sh("cargo fmt -- --check");
  await sh("git diff HEAD --exit-code --name-only");

  if (SHOULD_CARGO_PUBLISH) {
    await sh("cargo publish --dry-run");
  }

  const tagName = `v${version}`;
  await sh(`git tag -a ${tagName} -m "Release ${tagName}"`);
  await sh("git push origin master");
  await sh(`git push origin ${tagName}`);

  if (SHOULD_CARGO_PUBLISH) {
    await sh("cargo publish");
  }

  if (SHOULD_PUSH_DOCS_TO_GITHUB_PAGES) {
    await run("upload-docs");
  }
});

desc("Upload docs to GitHub Pages");
task("upload-docs", [], async () => {
  let origin_url;
  {
    const { code, output, error } = await shCapture(
      "git remote get-url origin",
    );
    if (code == 0) {
      origin_url = output.trim();
    } else {
      abort("Error getting origin's url from git");
    }
  }

  await sh("cargo clean --doc");
  await sh("cargo doc --no-deps");

  {
    const run = (command: string) => sh(command, { cwd: "target/doc" });

    await run("git init");
    await run("git add .");
    await run('git commit -am "(doc upload)"');
    await run(`git push -f ${origin_url} master:gh-pages`);
  }
});

run();

function isValidSemVer(s: string): boolean {
  return s.match(
    /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$/,
  ) != null;
}
