cmake_minimum_required(VERSION 3.15...3.31)

project(Koala VERSION 1.0
              DESCRIPTION "A Compiler for Koala language"
              LANGUAGES CXX)

file(GLOB_RECURSE SOURCE_FILES CONFIGURE_DEPENDS src/*.cpp)
file(GLOB_RECURSE HEADER_FILES CONFIGURE_DEPENDS src/*.hpp)

add_executable(koala main.cpp ${SOURCE_FILES} ${HEADER_FILES})
target_include_directories(koala PRIVATE src)

target_compile_options(koala PRIVATE "-ggdb" "-Wall" "-Weffc++" "-Wextra" "-Wconversion" "-Wsign-conversion" "-pedantic-errors" "-Werror" "-std=c++2b" "-Isrc")
