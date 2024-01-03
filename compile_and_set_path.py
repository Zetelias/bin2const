import os
import subprocess
import sys

# Compile the rust code in release mode
subprocess.run(["cargo", "build", "--release", "--bin", "bin2const"])

# Set the path to the compiled library
path_to_release = os.path.abspath("target/release")

match sys.platform:
    case "win32":
        subprocess.run(["setx", "PATH", f"{path_to_release};%PATH%"])
    case "linux":
        subprocess.run(["export", "PATH", f"{path_to_release}:$PATH"])
    case "darwin":
        subprocess.run(["export", "PATH", f"{path_to_release}:$PATH"])
    case _:
        print(f"Unsupported platform: \"{sys.platform}\", please update this script for your platform.")
        sys.exit(1)