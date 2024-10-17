cmake_minimum_required(VERSION 3.10)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(paw LANGUAGES CXX VERSION 0.1.0)

file(GLOB_RECURSE SOURCES src/*.cpp src/*.hpp)

include_directories(src)

add_executable(paw ${SOURCES})

if (MSVC)
   add_compile_options(/W4)
else()
   add_compile_options(-Wall -Wextra -Wpedantic -Werror)
endif()

target_link_libraries(paw)

