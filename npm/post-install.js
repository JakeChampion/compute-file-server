import { execFileSync } from "child_process";
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';
import { copyFile, writeFile } from "node:fs/promises";
import { endianness } from "node:os";
import { platform, arch } from "node:process";
const __dirname = dirname(fileURLToPath(import.meta.url));

const knownPackages = {
  "darwin arm64 LE": "@fastly/c-at-e-file-server-cli-darwin-arm64",
  "darwin x64 LE": "@fastly/c-at-e-file-server-cli-darwin-x64",
  "linux x64 LE": "@fastly/c-at-e-file-server-cli-linux-x64",
  "linux s390x LE": "@fastly/c-at-e-file-server-cli-linux-s390x",
  "win32 x64 LE": "@fastly/c-at-e-file-server-cli-win32-x64",
};

function pkgForCurrentPlatform() {
  let platformKey = `${platform} ${arch} ${endianness()}`;

  if (platformKey in knownPackages) {
    return knownPackages[platformKey];
  }
  throw new Error(`Unsupported platform: "${platformKey}". "@fastly/c-at-e-file-server-cli does not have a precompiled binary for the platform/architecture you are using. You can open an issue on https://github.com/JakeChampion/c-at-e-file-server/issues to request for your platform/architecture to be included."`);
}

const pkg = pkgForCurrentPlatform();

try {
  // First check for the binary package from our "optionalDependencies". This
  // package should have been installed alongside this package at install time.
  console.log({ pkg })
  const location = await import(pkg);
  console.log({ location: location.default })
  await copyFile(location.default, join(__dirname, 'c-at-e-file-server'))
  const contents = `export default "${location.default}";`
  console.log({ contents })
  await writeFile(join(__dirname, 'index.js'), contents, { encoding: 'utf-8' })
} catch (e) {
  console.error(e);
  throw new Error(`The package "${pkg}" could not be found, and is needed by @fastly/c-at-e-file-server-cli.
If you are installing @fastly/c-at-e-file-server-cli with npm, make sure that you don't specify the
"--no-optional" flag. The "optionalDependencies" package.json feature is used
by @fastly/c-at-e-file-server-cli to install the correct binary executable for your current platform.`);
}
