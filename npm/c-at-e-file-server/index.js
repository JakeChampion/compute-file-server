import { endianness } from "node:os";
import { platform, arch } from "node:process";

const knownPackages = {
  "win32 x64 LE": "@jakechampion/compute-file-server-win32-x64",
  "darwin arm64 LE": "@jakechampion/compute-file-server-darwin-arm64",
  "darwin x64 LE": "@jakechampion/compute-file-server-darwin-x64",
  "linux x64 LE": "@jakechampion/compute-file-server-linux-x64",
};

function pkgForCurrentPlatform() {
  let platformKey = `${platform} ${arch} ${endianness()}`;

  if (platformKey in knownPackages) {
    return knownPackages[platformKey];
  }
  throw new Error(`Unsupported platform: "${platformKey}". "@jakechampion/compute-file-server does not have a precompiled binary for the platform/architecture you are using. You can open an issue on https://github.com/JakeChampion/compute-file-server/tree/main/npm/issues to request for your platform/architecture to be included."`);
}

const pkg = pkgForCurrentPlatform();

let location;
try {
  // Check for the binary package from our "optionalDependencies". This
  // package should have been installed alongside this package at install time.
  location = (await import(pkg)).default;
} catch (e) {
  throw new Error(`The package "${pkg}" could not be found, and is needed by @jakechampion/compute-file-server.
If you are installing @jakechampion/compute-file-server with npm, make sure that you don't specify the
"--no-optional" flag. The "optionalDependencies" package.json feature is used
by @jakechampion/compute-file-server to install the correct binary executable for your current platform.`);
}

export default location;