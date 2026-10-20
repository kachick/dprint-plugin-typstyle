/**
 * This script checks for any typstyle updates and then
 * automatically publishes a new version of the plugin if so.
 */
import { $, CargoToml, semver } from "automation";
import { aiFixTypstyleUpdate } from "./ai_fix.ts";

const crateName = "typstyle-core";
// typstyle-core doesn't re-export typst-syntax, so this is kept on the version it depends on
const syntaxCrateName = "typst-syntax";
const rootDirPath = $.path(import.meta.dirname!).parentOrThrow();
const cargoToml = new CargoToml(rootDirPath.join("Cargo.toml"));
const currentVersion = getCargoTomlVersion(cargoToml.text(), crateName);

$.logStep("Getting latest version...");
const latestVersion = await getLatestVersion();
if (semver.compare(semver.parse(currentVersion), semver.parse(latestVersion)) >= 0) {
  $.log("No new update found. Exiting.");
  Deno.exit(0);
}

$.log(`Found new version: ${currentVersion} -> ${latestVersion}`);
$.logStep("Updating rust-toolchain.toml...");
await updateRustToolchain(latestVersion);
$.logStep("Updating Cargo.toml...");
const isPatchBump = semver.parse(currentVersion).major === semver.parse(latestVersion).major
  && semver.parse(currentVersion).minor === semver.parse(latestVersion).minor;
cargoToml.replaceAll(`${crateName} = "${currentVersion}"`, `${crateName} = "${latestVersion}"`);
await updateSyntaxCrate(latestVersion);

// Verify the update. A clean patch bump publishes exactly as before. A minor
// bump always gets an AI review (typstyle may have added options without
// breaking the build), and a patch bump that fails the checks gets an AI fix attempt.
$.logStep("Running checks (test + clippy + wasm build)...");
const checks = await runChecks();

if (!isPatchBump || !checks.passed) {
  if (checks.passed) {
    $.logStep("Minor typstyle update — running AI review for new/changed options...");
  } else {
    $.logStep(`${isPatchBump ? "Patch" : "Minor"} update failed the checks — running AI fix...`);
  }
  await aiFixTypstyleUpdate({
    isPatchBump,
    fromVersion: currentVersion,
    toVersion: latestVersion,
    checksPassed: checks.passed,
    // hand the failing output to the AI so it can go straight to fixing
    // instead of re-running the checks just to rediscover the errors.
    checkOutput: checks.output,
  });

  // the AI must leave the project in a passing state, otherwise fail the
  // workflow (nothing gets published and the maintainer is notified).
  $.logStep("Re-running checks after AI changes...");
  await assertChecks();
}

if (Deno.args.includes("--skip-publish")) {
  Deno.exit(0);
}

$.logStep(`Committing typstyle version bump commit...`);
await $`git add .`;
const message = `${isPatchBump ? "fix" : "feat"}: update to typstyle ${latestVersion}`;
await $`git commit -m ${message}`;

$.logStep("Bumping version in Cargo.toml...");
// reload from disk before bumping: the AI may have edited Cargo.toml, and the
// in-memory copy loaded at startup is stale. Writing the stale copy here would
// clobber those edits.
const releaseCargoToml = new CargoToml(rootDirPath.join("Cargo.toml"));
releaseCargoToml.bumpCargoTomlVersion(isPatchBump ? "patch" : "minor");

// release
const newVersion = releaseCargoToml.version();
$.logStep(`Committing and publishing ${newVersion}...`);
await $`git add .`;
await $`git commit -m ${newVersion}`;
await $`git push origin main`;
await $`git tag ${newVersion}`;
await $`git push origin ${newVersion}`;

// the checks that must pass before publishing. clippy is included because
// Codex runs it with warnings denied, so a clippy warning is as breaking as a
// test failure, and the wasm release build is included because that is what
// actually ships. `inheritPiped` + `captureCombined` streams the output to the
// CI log live while also capturing it so a failure can be handed to the AI.
async function runChecks(): Promise<{ passed: boolean; output: string }> {
  const results = [];
  for (const command of checkCommands()) {
    results.push(await capture(command()));
  }
  const failures = results.filter((r) => r.code !== 0);
  return {
    passed: failures.length === 0,
    output: failures.map((r) => r.combined).join("\n\n"),
  };
}

function capture(command: ReturnType<typeof $>) {
  return command
    .stdout("inheritPiped")
    .stderr("inheritPiped")
    .captureCombined()
    .noThrow();
}

// same checks as `runChecks`, but throws on the first failure so the workflow
// aborts before anything is committed, tagged, or published.
async function assertChecks(): Promise<void> {
  for (const command of checkCommands()) {
    await command();
  }
}

// a function (not a top-level const) so it is hoisted -- `runChecks` is called
// from top-level code above before a const declared here would be initialized.
function checkCommands() {
  return [
    () => $`cargo test`,
    () => $`cargo clippy --all-targets --all-features -- -D warnings`,
    () => $`cargo build --target wasm32-unknown-unknown --features wasm --release`,
  ];
}

function getCargoTomlVersion(text: string, name: string) {
  const match = text.match(new RegExp(`^${name} = "([^"]+)"`, "m"));
  const version = match?.[1];
  if (version == null) {
    throw new Error(`Could not find ${name} version in Cargo.toml.`);
  }
  $.logLight(`Found ${name} version in Cargo.toml:`, version);
  return version;
}

async function getLatestVersion() {
  const data = await getCrateData(`https://crates.io/api/v1/crates/${crateName}`);
  const version = data.crate?.max_stable_version;
  if (version == null) {
    throw new Error(`Could not find latest version of ${crateName} on crates.io.`);
  }
  $.logLight(`Latest ${crateName} version on crates.io:`, version);
  return version;
}

async function updateSyntaxCrate(typstyleVersion: string) {
  const data = await getCrateData(`https://crates.io/api/v1/crates/${crateName}/${typstyleVersion}/dependencies`);
  const requirement = data.dependencies?.find((d) => d.crate_id === syntaxCrateName && d.kind === "normal")?.req;
  // the requirement is something like "^0.15.1"
  const requiredVersion = requirement?.match(/\d+\.\d+\.\d+/)?.[0];
  if (requiredVersion == null) {
    throw new Error(`Could not find the ${syntaxCrateName} requirement of ${crateName} ${typstyleVersion}.`);
  }
  const localVersion = getCargoTomlVersion(cargoToml.text(), syntaxCrateName);
  if (semver.greaterThan(semver.parse(requiredVersion), semver.parse(localVersion))) {
    $.log(`Updating ${syntaxCrateName}: ${localVersion} -> ${requiredVersion}`);
    cargoToml.replaceAll(`${syntaxCrateName} = "${localVersion}"`, `${syntaxCrateName} = "${requiredVersion}"`);
  }
}

async function updateRustToolchain(typstyleVersion: string) {
  const data = await getCrateData(`https://crates.io/api/v1/crates/${crateName}/${typstyleVersion}`);
  const requiredRustVersion = data.version?.rust_version;
  if (requiredRustVersion == null) {
    $.log(`${crateName} ${typstyleVersion} does not declare a rust_version; leaving rust-toolchain.toml alone.`);
    return;
  }

  const toolchainPath = rootDirPath.join("rust-toolchain.toml");
  const localContent = toolchainPath.readTextSync();
  const localMatch = localContent.match(/channel\s*=\s*"([^"]+)"/);
  if (localMatch == null) {
    throw new Error("Could not find channel in local rust-toolchain.toml.");
  }
  // crates.io rust_version may be "1.89" (no patch), so pad it to
  // MAJOR.MINOR.PATCH for @std/semver to parse it
  const normalize = (v: string) => /^\d+\.\d+$/.test(v) ? `${v}.0` : v;
  const local = semver.parse(normalize(localMatch[1]));
  const required = semver.parse(normalize(requiredRustVersion));
  if (semver.greaterThan(required, local)) {
    $.log(`Updating Rust toolchain: ${localMatch[1]} -> ${requiredRustVersion}`);
    toolchainPath.writeTextSync(localContent.replace(localMatch[0], `channel = "${requiredRustVersion}"`));
  } else {
    $.log(
      `Rust toolchain at ${
        localMatch[1]
      } already satisfies ${crateName} ${typstyleVersion} (needs >= ${requiredRustVersion}).`,
    );
  }
}

function getCrateData(url: string) {
  return $.request(url)
    .header("User-Agent", "dprint-plugin-typstyle update script")
    .json<{
      crate?: { max_stable_version?: string };
      version?: { rust_version?: string | null };
      dependencies?: { crate_id: string; req: string; kind: string }[];
    }>();
}
