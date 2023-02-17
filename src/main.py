"""Wrapper around the rust executable file"""

import subprocess

subprocess.run(["../target/release/algo_projet_s6", "exemple_1.pts"])
