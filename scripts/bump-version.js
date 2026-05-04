#!/usr/bin/env node

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { execSync } from "child_process";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const arg = process.argv[2];

if (!arg) {
  console.error(
    "Usage: node scripts/bump-version.js <major|minor|patch|X.Y.Z[-suffix]>",
  );
  process.exit(1);
}

function readCurrentVersion() {
  const pkg = JSON.parse(
    fs.readFileSync(path.join(root, "package.json"), "utf8"),
  );
  return pkg.version;
}

function bump(current, kind) {
  const match = current.match(/^(\d+)\.(\d+)\.(\d+)(?:-([\w.]+))?$/);
  if (!match) {
    console.error(`Cannot parse current version: ${current}`);
    process.exit(1);
  }
  let [, maj, min, patch] = match;
  maj = Number(maj);
  min = Number(min);
  patch = Number(patch);
  if (kind === "major") {
    maj += 1;
    min = 0;
    patch = 0;
  } else if (kind === "minor") {
    min += 1;
    patch = 0;
  } else if (kind === "patch") {
    patch += 1;
  } else {
    console.error(`Unknown bump kind: ${kind}`);
    process.exit(1);
  }
  return `${maj}.${min}.${patch}`;
}

const current = readCurrentVersion();
let version;
if (["major", "minor", "patch"].includes(arg)) {
  version = bump(current, arg);
} else if (/^\d+\.\d+\.\d+(-[\w.]+)?$/.test(arg)) {
  version = arg;
} else {
  console.error(`Invalid version: ${arg}`);
  console.error("Expected: major | minor | patch | X.Y.Z[-suffix]");
  process.exit(1);
}

console.log(`Bumping ${current} → ${version}`);

const changed = [];

function writeJson(filePath, mutator) {
  const raw = fs.readFileSync(filePath, "utf8");
  const data = JSON.parse(raw);
  mutator(data);
  const out = JSON.stringify(data, null, 2) + (raw.endsWith("\n") ? "\n" : "");
  fs.writeFileSync(filePath, out);
  changed.push(path.relative(root, filePath));
}

function writeText(filePath, mutator) {
  const original = fs.readFileSync(filePath, "utf8");
  const updated = mutator(original);
  if (updated !== original) {
    fs.writeFileSync(filePath, updated);
    changed.push(path.relative(root, filePath));
  }
}

writeJson(path.join(root, "package.json"), (p) => {
  p.version = version;
});

const pkgLockPath = path.join(root, "package-lock.json");
if (fs.existsSync(pkgLockPath)) {
  writeJson(pkgLockPath, (p) => {
    p.version = version;
    if (p.packages && p.packages[""]) {
      p.packages[""].version = version;
    }
  });
}

const cargoLockPath = path.join(root, "Cargo.lock");
if (fs.existsSync(cargoLockPath)) {
  writeText(cargoLockPath, (content) => {
    return content
      .replace(
        /(name = "mangofetch-cli"\nversion = ")([^"]+)(")/,
        (_, a, _b, c) => `${a}${version}${c}`,
      )
      .replace(
        /(name = "mangofetch-core"\nversion = ")([^"]+)(")/,
        (_, a, _b, c) => `${a}${version}${c}`,
      )
      .replace(
        /(name = "mangofetch-plugin-sdk"\nversion = ")([^"]+)(")/,
        (_, a, _b, c) => `${a}${version}${c}`,
      );
  });
}

function updateCargoToml(pkgDir) {
  const p = path.join(root, pkgDir, "Cargo.toml");
  if (!fs.existsSync(p)) return;
  writeText(p, (content) => {
    return content.replace(/^version\s*=\s*"[^"]+"/m, `version = "${version}"`);
  });
}

updateCargoToml("mangofetch-cli");
updateCargoToml("mangofetch-core");
updateCargoToml("mangofetch-plugin-sdk");

// Metainfo was removed or renamed, skipping for now as it's not in root
// const metainfoAbs = path.join(root, "flatpak", "wtf.tonho.mangofetch.metainfo.xml");
if (hasBinary("appstreamcli")) {
  try {
    execSync(`appstreamcli validate "${metainfoAbs}"`, { stdio: "inherit" });
    console.log("appstreamcli validate: OK");
  } catch {
    console.error("appstreamcli validate failed — aborting.");
    process.exit(1);
  }
} else {
  console.warn(
    "appstreamcli not found in PATH — skipping metainfo validation.",
  );
}

console.log(`v${version}`);
