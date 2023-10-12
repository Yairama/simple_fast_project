import os
import subprocess

def run():
    dir_path = os.path.dirname(os.path.realpath(__file__))
    exe_path = os.path.join(dir_path, "sfp.exe")
    subprocess.run([exe_path])

