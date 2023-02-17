"""Wrapper around the rust executable file"""

import subprocess
import sys

subprocess.run(["../target/release/algo_projet_s6", sys.argv[1]])
