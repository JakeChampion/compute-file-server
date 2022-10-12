const platformPackages = [
  "@fastly/c-at-e-file-server-cli-darwin-arm64",
  "@fastly/c-at-e-file-server-cli-darwin-x64",
  "@fastly/c-at-e-file-server-cli-linux-x64",
  "@fastly/c-at-e-file-server-cli-linux-s390x",
  "@fastly/c-at-e-file-server-cli-win32-x64",
]

let location;
for (const pkg of platformPackages) {
  try {
    location = await import(pkg);
  } catch {}
}
throw new Error(
  "@fastly/c-at-e-file-server-cli does not have a precompiled binary for the platform/architecture you are using. You can open an issue on https://github.com/JakeChampion/c-at-e-file-server/issues to request for your platform/architecture to be included."
);

export default location;