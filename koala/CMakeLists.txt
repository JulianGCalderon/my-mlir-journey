cmake_minimum_required(VERSION 3.15...3.31)

project(Koala VERSION 1.0
              DESCRIPTION "A Compiler for Koala language"
              LANGUAGES CXX)

find_package(LLVM REQUIRED CONFIG)
message(STATUS "Found LLVM ${LLVM_PACKAGE_VERSION}")
message(STATUS "Using LLVMConfig.cmake in: ${LLVM_DIR}")

include_directories(SYSTEM ${LLVM_INCLUDE_DIRS})
include_directories("src")

separate_arguments(LLVM_DEFINITIONS_LIST NATIVE_COMMAND ${LLVM_DEFINITIONS})
add_definitions(${LLVM_DEFINITIONS_LIST})

file(GLOB_RECURSE SOURCE_FILES CONFIGURE_DEPENDS src/*.cpp)
file(GLOB_RECURSE HEADER_FILES CONFIGURE_DEPENDS src/*.hpp)
add_executable(koala main.cpp ${SOURCE_FILES} ${HEADER_FILES})

llvm_map_components_to_libnames(llvm_libs ${LLVM_TARGETS_TO_BUILD})
target_link_libraries(koala ${llvm_libs})

target_compile_options(koala PRIVATE
              "-ggdb" "-Wall" "-Weffc++"
              "-Wextra" "-Wconversion" "-Wsign-conversion"
              "-pedantic-errors" "-Werror" "-std=c++2b" 
)
