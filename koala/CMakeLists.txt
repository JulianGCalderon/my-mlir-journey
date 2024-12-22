cmake_minimum_required(VERSION 3.15...3.31)

project(Koala VERSION 1.0
              DESCRIPTION "A Compiler for Koala language"
              LANGUAGES CXX)

add_compile_options(
              "-ggdb" "-Wall" "-Weffc++"
              "-Wextra" "-Wconversion" "-Wsign-conversion"
              "-pedantic-errors" "-Werror" "-std=c++2b"
)
