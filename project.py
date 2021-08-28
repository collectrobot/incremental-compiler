from os import getcwd, chdir
from subprocess import run as call_extern
from shutil import copyfile 

import argparse
import sys

top_level_dir = getcwd()

compiler_dir = top_level_dir + "/" + "compiler"

runtime_dir = top_level_dir + "/" + "runtime"

def build_runtime():

    print("building runtime")

    # need to chdir to make cargo build work
    chdir(runtime_dir)

    print("invoking cargo build in: " + runtime_dir)
    try:
        call_extern(["cargo", "build", "--release"])
        print("success")
    except BaseException as e:
        print("build failed: {}".format(e))

    # try to copy static library to bin folder in compiler project
    static_lib_src = runtime_dir + "/target" + "/release" + "/runtime.lib"
    static_lib_dst = compiler_dir + "/src" + "/backend" + "/bin_include" + "/win64" + "/runtime.lib"
    print(
        "copying static library:\n" +
        "src: " + static_lib_src + "\n" +
        "dst: " + static_lib_dst
    )

    try:
        copyfile(
            static_lib_src,
            static_lib_dst
        )
        print("success")
    except BaseException as e:
        print("copying failed: {}".format(e))

    # back to top level
    chdir(top_level_dir)

    print("")

def build_compiler(run = False):
    # need to chdir to make cargo build work
    chdir(compiler_dir)

    print("invoking cargo build in: " + compiler_dir)

    build_or_run = "build"
    if run == True:
        build_or_run = "run"

    print(("running" if build_or_run == "run" else "building") + " the compiler")

    try:
        call_extern(["cargo", build_or_run])
        print("success")
    except BaseException as e:
        print(build_or_run + " failed: {}".format(e))

def setup_argparse():
    parser = argparse.ArgumentParser(argument_default=argparse.SUPPRESS)

    parser.add_argument("--run", action="store_true")
    parser.add_argument("--test", action="store_true")

    parser.parse_args()

if __name__ == "__main__":

    args = setup_argparse()

    run_compiler = False

    if "run" in args:
        run_compiler = True

    build_runtime()
    build_compiler(run_compiler)