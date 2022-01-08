from os import getcwd, chdir, path, remove as remove_file
from subprocess import run as call_extern
from shutil import copyfile 
from platform import system

import argparse

os_kind = system()

top_level_dir = getcwd()

compiler_dir = top_level_dir + "/" + "compiler"

runtime_dir = top_level_dir + "/" + "runtime"

def build_runtime(args):

    success = True

    build_directory = "/" + args["build_type"] # either /release or /debug
    cargo_build_type = args["build_type"]
    if cargo_build_type == "debug":
        cargo_build_type = "" # --debug isn't a cargo flag, however, --release is
    else:
        cargo_build_type = "--" + cargo_build_type

    print("building runtime")

    # need to chdir to make cargo build work
    chdir(runtime_dir)

    static_lib_src = runtime_dir + "/target" + build_directory + "/runtime.lib"
    static_lib_fully_linked = runtime_dir + "/target" + build_directory + "/runtime_fully_linked.lib"

    print("invoking cargo build in: " + runtime_dir)
    try:
        call_extern([x for x in ["cargo", "build", cargo_build_type] if x])
        print("success")
    except BaseException as e:
        print("build failed: {}".format(e))
        success = False

    # need to combine all static libraries into one library so that later when the compiler invokes link.exe
    # to link the runtime.lib and <filename>.obj, we don't get a bunch of external symbols unresolved
    # this step requires lib.exe to be reachable from the command line
    #
    # for the runtime, rustc currently reports that the following static libraries must be linked against:
    # kernel32.lib ws2_32.lib advapi32.lib userenv.lib kernel32.lib msvcrt.lib
    # (rustc --crate-type=staticlib --print=native-static-libs .\static_lib_check.rs)
    # however, by trial and error I've figured out that I need to replace msvcrt.lib with libcmt.lib libucrt.lib

    if success:

        lib_list = ["{}".format(static_lib_src), "kernel32.lib", "ws2_32.lib", "advapi32.lib", "userenv.lib", "libcmt.lib", "libucrt.lib"]

        print(
            "combining static libraries: {}\n".format(lib_list) +
            "into: {}".format(static_lib_fully_linked)
        )

        print(["lib", "/out:{}".format(static_lib_fully_linked)] + lib_list)

        try:
            call_extern(["lib", "/out:{}".format(static_lib_fully_linked)] + lib_list)
        except BaseException as e:
            print("failed to combine static libraries: {}".format(e))
            success = False

    if success:
        static_lib_dst = compiler_dir + "/src" + "/backend" + "/bin_include" + "/win64" + "/runtime.lib"
        print(
            "copying static library:\n" +
            "src: " + static_lib_fully_linked + "\n" +
            "dst: " + static_lib_dst
        )

        try:
            copyfile(
                static_lib_fully_linked,
                static_lib_dst
            )
            print("success")
        except BaseException as e:
            print("copying failed: {}".format(e))
            success = False

    # back to top level
    chdir(top_level_dir)

    print("")

    return success

def build_compiler(args):
    # need to chdir to make cargo build work
    chdir(compiler_dir)

    cargo_invoke = args["op"]

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

    parser.add_argument("--op", help="build, run, or test the compiler", nargs=1, choices=("build", "run", "test"), default="build")
    parser.add_argument("--build-type", help="release or debug build", nargs=1, choices=("release", "debug"), default="debug")

    args = parser.parse_args()

    args = vars(args)

    de_list_args = {}

    for arg in args:
        if isinstance(args[arg], list):
            de_list_args[arg] = args[arg][0]
        else:
            de_list_args[arg] = args[arg]

    return de_list_args

if __name__ == "__main__":

    args = do_argparse()
    if (os_kind == "Windows"):

        if (args["op"] == "build"):
            build_runtime(args)

        build_compiler(args)

    else:
        print("{} is not supported yet.".format(os_kind))