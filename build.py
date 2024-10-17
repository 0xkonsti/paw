#!/usr/bin/env python3

import os
import sys
from shutil import which, rmtree
from termcolor import colored

BUILD_DIR: str = "build"
EXEC_NAME: str = "paw"
NINJA: str = "-G Ninja"

CXX_COMPILER_PRIORITY = ["clang++", "g++", "msvc"]


def find_cxx_compiler() -> str:
    for compiler in CXX_COMPILER_PRIORITY:
        if which(compiler) is not None:
            return compiler

    raise FileNotFoundError("No C++ compiler found on PATH")


CXX_COMPILER: str = f"-DCMAKE_CXX_COMPILER:FILEPATH={find_cxx_compiler()}"


def format_project() -> None:
    if which("clang-format") is not None:
        files = ""
        for path, _, filenames in os.walk("src"):
            for filename in filenames:
                if (
                    filename.endswith(".cpp")
                    or filename.endswith(".hpp")
                    or filename.endswith(".h")
                    or filename.endswith(".c")
                ):
                    files += f"{os.path.join(path, filename)} "

        for path, _, filenames in os.walk("include"):
            for filename in filenames:
                if (
                    filename.endswith(".cpp")
                    or filename.endswith(".hpp")
                    or filename.endswith(".h")
                    or filename.endswith(".c")
                ):
                    files += f"{os.path.join(path, filename)} "

        cf = "clang-format"
        cf_color = colored(cf, "blue", attrs=["bold"])

        def cmd(cf):
            return f"{cf} -i -style=file {files}"

        print("")
        print(cmd(cf_color))
        print("")
        os.system(cmd(cf))


def cmake_configure(build_type: str) -> None:
    cmd = f"cmake -DCMAKE_BUILD_TYPE:STRING={build_type} {CXX_COMPILER} -S . -B {BUILD_DIR} {NINJA}"

    print("")
    print(cmd)
    print("")
    os.system(cmd)


def cmake_build() -> None:
    cmd = f"cmake --build {BUILD_DIR}"

    print("")
    print(cmd)
    print("")
    os.system(cmd)


def run_file(path: str) -> None:
    cmd = f"./{BUILD_DIR}/{path}"
    if os.name == "nt":
        cmd = f"{BUILD_DIR}\\{path}"

    os.system(cmd)


def dev() -> None:
    # search for the executable (run or run.exe) in the build (or build\Debug or build\Release) directory

    for _, _, files in os.walk(BUILD_DIR):
        for file in files:
            print(file)
            if file == EXEC_NAME or file == f"{EXEC_NAME}.exe":
                print(f"Running {file}")
                run_file(file)
                return


def main() -> None:

    argv = sys.argv[1:]

    # Default to formatting the project
    if len(argv) == 0:
        format_project()
        cmake_configure("Debug")
        cmake_build()
        return

    if argv[0] == "fmt":
        format_project()

    if argv[0] == "release":
        rmtree(BUILD_DIR)
        format_project()
        cmake_configure("Release")
        cmake_build()

    if argv[0] == "debug":
        format_project()
        cmake_configure("Debug")
        cmake_build()

    if argv[0] == "clean":
        rmtree(BUILD_DIR)

    if argv[0] == "rebuild":
        rmtree(BUILD_DIR)
        format_project()
        cmake_configure("Debug")
        cmake_build()

    if argv[0] == "dev":
        format_project()
        cmake_configure("Debug")
        cmake_build()
        dev()


if __name__ == "__main__":
    main()
