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

def build_compiler(parsed_args):
    # need to chdir to make cargo build work
    chdir(compiler_dir)

    cargo_invoke = parsed_args["op"][0]

    print("invoking cargo " + cargo_invoke + " in: " + compiler_dir)

    print(
        "running" if cargo_invoke == "run" else "building" if cargo_invoke == "build" else "testing" +
        " the compiler"
    )

    try:
        call_extern(["cargo", cargo_invoke])
        print("success")
    except BaseException as e:
        print(cargo_invoke + " failed: {}".format(e))

def do_argparse():
    parser = argparse.ArgumentParser(argument_default=argparse.SUPPRESS)

    parser.add_argument("--op", help="build, run, or test the compiler", nargs=1, choices=("build", "run", "test"))

    args = parser.parse_args()

    args = vars(args)

    if len(args) == 0:
        args["op"] = ["build"]
    
    return args

if __name__ == "__main__":

    args = do_argparse()

    build_runtime()
    build_compiler(args)