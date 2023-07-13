import subprocess
import sys
import re


def main(args):
    version = args[0]

    changelog_contents = open("CHANGELOG.md").read()
    first_version = changelog_contents.split("## Version", 1)[1].split(" - ")[0].strip()
    if first_version != version:
        print(f"Error: Missing CHANGELOG.md section for {version}", file=sys.stderr)
        return 1

    # Check python formatting
    subprocess.run(["black", "--check", "."], check=True)

    # Check python linting
    subprocess.run(["ruff", "check", "."], check=True)

    # Check cargo clippy
    subprocess.run(["cargo", "clippy"], cwd="rust", check=True)

    pyproject_contents = open("pyproject.toml").read()
    pyproject_contents = re.sub('version = ".*"', f'version = "{version}"', pyproject_contents)
    open("pyproject.toml", "w").write(pyproject_contents)

    subprocess.run(["git", "add", "pyproject.toml", "CHANGELOG.md"], check=True)
    subprocess.run(["git", "commit", "--edit", f"--message=Bump to version {version}"], check=True)
    subprocess.run(["git", "tag", version], check=True)

    return 0


sys.exit(main(sys.argv[1:]))
