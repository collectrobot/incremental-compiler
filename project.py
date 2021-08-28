from os import getcwd, chdir
from subprocess import run as call_extern
from shutil import copyfile 

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

def build_compiler():
    # need to chdir to make cargo build work
    chdir(compiler_dir)

    print("building compiler")

    print("invoking cargo build in: " + compiler_dir)

    try:
        call_extern(["cargo", "build"])
        print("success")
    except BaseException as e:
        print("build failed: {}".format(e))

if __name__ == "__main__":
    build_runtime()
    build_compiler()